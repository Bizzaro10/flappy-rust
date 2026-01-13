use crate::components::*;
use crate::constants::*;
use crate::resources::*;
use crate::utils::*;
use bevy::prelude::*;
use rand::distributions::WeightedIndex;
use rand::prelude::Distribution;
use rand::thread_rng;
use crate::nn::Net;
use crate::constants::NUM_BIRDS;
pub fn blink_space_bar_text(
    time: Res<Time>,
    mut query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
) {
    let (mut space, mut visibility) = query.single_mut();

    let timer = &mut space.0;
    timer.tick(time.delta());

    if timer.finished() {
        if *visibility == Visibility::Hidden {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}

pub fn move_background(time: Res<Time>, mut query: Query<&mut Transform, With<Background>>) {
    let mut background_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 20. * delta;

    background_transform.translation.x -= delta_x;

    if background_transform.translation.x < -288.0 {
        background_transform.translation.x = 0.;
    }
}

pub fn move_ground(time: Res<Time>, mut query: Query<&mut Transform, With<Ground>>) {
    let mut ground_transform = query.single_mut();
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta; // move faster because it's closer to the camera perspective

    ground_transform.translation.x -= delta_x;

    if ground_transform.translation.x < -288.0 {
        ground_transform.translation.x = 0.;
    }
}

pub fn animate_bird(time: Res<Time>, mut query: Query<(&mut Bird, &mut TextureAtlas)>) {
    for (mut bird, mut texture_atlas) in query.iter_mut() {
        let delta = time.delta();

        bird.timer.tick(delta);

        if bird.timer.finished() {
            texture_atlas.index = if texture_atlas.index == 2 {
                0
            } else {
                texture_atlas.index + 1
            };
        }
    }
}

pub fn start_game(
    mut game: ResMut<Game>,
    mut space_query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
    mut game_over_query: Query<&mut Visibility, (With<GameOverText>, Without<PressSpaceBarText>)>,
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut upper_pipe_query: Query<(&mut Transform, &mut UpperPipe), (With<UpperPipe>, Without<Bird>)>,
    mut lower_pipe_query: Query<
        &mut Transform,
        (With<LowerPipe>, Without<Bird>, Without<UpperPipe>),
    >,
) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    // Set game state to Active when starting
    game.state = GameState::Active;

    if game.state == GameState::GameOver {
        for (i, (mut transform, mut upper_pipe)) in upper_pipe_query.iter_mut().enumerate() {
            let delta_x = i as f32 * 200.0 + 200.;

            upper_pipe.passed = false;
            transform.translation.x = 0.;
            transform.translation.x += delta_x;
        }

        for (i, mut transform) in lower_pipe_query.iter_mut().enumerate() {
            let delta_x = i as f32 * 200.0 + 200.;

            transform.translation.x = 0.;
            transform.translation.x += delta_x;
        }
    };

    for (mut bird, mut transform) in bird_query.iter_mut() {
        bird.velocity = 0.0;
        transform.translation.y = 0.0;
        transform.rotation = Quat::from_rotation_z(0.0);
    }

    let (mut space, mut visibility) = space_query.single_mut();
    space.0.reset();
    *visibility = Visibility::Hidden;

    let mut game_over_visibility = game_over_query.single_mut();
    *game_over_visibility = Visibility::Hidden;
}

pub fn gravity(
    time: Res<Time>,
    mut game: ResMut<Game>,
    mut query: Query<(&mut Bird, &mut Transform)>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sim_state: Res<SimulationState>,
) {
    for (mut bird, mut transform) in query.iter_mut() {
        if bird.is_dead { continue; }
        let delta = time.delta().as_secs_f32();
        let gravity = 9.8;
        let delta_v = gravity * 150. * delta;
        let delta_y = bird.velocity * delta;
        let new_y = (transform.translation.y + delta_y).min(260.0);

        transform.translation.y = new_y;

        bird.velocity -= delta_v;
        transform.translation.y += bird.velocity * delta;

        // Rotate the bird
        let rotation = bird.velocity / 600.0;
        let max_rotation = 0.5;
        transform.rotation = Quat::from_rotation_z(rotation.max(-max_rotation).min(max_rotation));

        let ground_y = -250.0;
        let ground_height = 112.0;
        let bird_height = 23.0;

        let collision_point = ground_y + ground_height / 2.0 + bird_height / 2.0;

        if transform.translation.y < collision_point {
            transform.translation.y = collision_point;
            bird.velocity = 0.0;

            if sim_state.mode == GameMode::Human {
                game.state = GameState::GameOver;
                *game_over_query.single_mut() = Visibility::Visible;

                // play game over sound
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/hit.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
            } else {
                 if !bird.is_dead {
                    bird.is_dead = true;
                    // Move bird way off screen so it's not visible
                    transform.translation.y = -1000.0; 
                }
            }
        }
    }
}

// systems.rs

pub fn jump(
    mut query: Query<&mut Bird>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sim_state: Res<SimulationState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if sim_state.mode == GameMode::AI { return; }
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }

    commands.spawn(AudioBundle {
        source: asset_server.load("audio/wing.ogg"),
        settings: PlaybackSettings::DESPAWN,
        ..default()
    });

    for mut bird in query.iter_mut() {
        if bird.is_dead { continue; }
        bird.velocity = 300.0;
    }
}

pub fn pipes(
    time: Res<Time>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &mut Transform)>,
    mut lower_pipe_query: Query<(&LowerPipe, &mut Transform), Without<UpperPipe>>,
    mut bird_query: Query<(&mut Bird, &mut Transform), (With<Bird>, Without<LowerPipe>, Without<UpperPipe>)>,
    mut game_over_query: Query<&mut Visibility, With<GameOverText>>,
    asset_server: Res<AssetServer>,
    mut game: ResMut<Game>,
    mut commands: Commands,
    sim_state: Res<SimulationState>,
) {
    let delta = time.delta().as_secs_f32();
    let delta_x = 150. * delta;

    let utmost_right_pipe = upper_pipe_query
        .iter()
        .max_by(|(_, a), (_, b)| a.translation.x.partial_cmp(&b.translation.x).unwrap())
        .unwrap()
        .1
        .translation
        .x;

    let new_pipe_position = utmost_right_pipe + 200.0;
    let (lower_y, upper_y) = random_pipe_position();
    let out_of_screen_x = (-WINDOW_WIDTH / 2.) - 26.;

    for (mut upper_pipe, mut transform) in upper_pipe_query.iter_mut() {
        transform.translation.x -= delta_x;

        if transform.translation.x < out_of_screen_x {
            transform.translation.x = new_pipe_position;
            transform.translation.y = upper_y;
            upper_pipe.passed = false;
        }
    }

    for (_, mut transform) in lower_pipe_query.iter_mut() {
        transform.translation.x -= delta_x;

        if transform.translation.x < out_of_screen_x {
            transform.translation.x = new_pipe_position;
            transform.translation.y = lower_y;
        }
    }

    let is_collision = |bird_transform: &Transform, pipe_transform: &Transform| -> bool {
        let bird_x = bird_transform.translation.x;
        let bird_y = bird_transform.translation.y;
        let bird_width = 34.0;
        let bird_height = 24.0;

        let pipe_x = pipe_transform.translation.x;
        let pipe_y = pipe_transform.translation.y;
        let pipe_width = 52.0;
        let pipe_height = 320.0;

        let collision_x = bird_x + bird_width / 2.0 > pipe_x - pipe_width / 2.0
            && bird_x - bird_width / 2.0 < pipe_x + pipe_width / 2.0;
        let collision_y = bird_y + bird_height / 2.0 > pipe_y - pipe_height / 2.0
            && bird_y - bird_height / 2.0 < pipe_y + pipe_height / 2.0;

        collision_x && collision_y
    };

    for (mut bird, mut bird_transform) in bird_query.iter_mut() {
        if bird.is_dead { continue; }
        
        let mut collided = false;

        for (_, transform) in upper_pipe_query.iter_mut() {
            if is_collision(&bird_transform, &transform) {
                collided = true;
                break;
            }
        }

        if !collided {
            for (_, transform) in lower_pipe_query.iter_mut() {
                if is_collision(&bird_transform, &transform) {
                    collided = true;
                    break;
                }
            }
        }

        if collided {
            if sim_state.mode == GameMode::Human {
                game.state = GameState::GameOver;
                *game_over_query.single_mut() = Visibility::Visible;

                // Play game over sound
                commands.spawn(AudioBundle {
                    source: asset_server.load("audio/hit.ogg"),
                    settings: PlaybackSettings::DESPAWN,
                    ..default()
                });
            } else {
                bird.is_dead = true;
                // Move bird off screen so it doesn't form a vertical line
                bird_transform.translation.y = -1000.0;
            }
        }
    }
}

pub fn score(
    mut game: ResMut<Game>,
    mut bird_query: Query<(&mut Bird, &Transform)>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    sim_state: Res<SimulationState>,
) {
    for (mut bird, bird_transform) in bird_query.iter_mut() {
        for (mut upper_pipe, transform) in upper_pipe_query.iter_mut() {
            let passed = transform.translation.x < bird_transform.translation.x;
            let passed_state = upper_pipe.passed;

            if passed && !passed_state {
                game.score += 1;
                upper_pipe.passed = true;
                
                // Reward fitness for passing a pipe
                bird.fitness += 20.0;

                if game.score % 10 == 0 { // Print less frequently
                    println!("Score: {}", game.score);
                }

                // Play sound only in Human mode to avoid noise
                // We don't have direct access to SimulationState here... 
                // Wait, we need to add sim_state to arguments.
                
                // Let's modify the function signature first in a separate step or try to assume we added it.
                // The tool call below this will add the argument.
                // For now, I will modify the body assuming `sim_state` exists.
                if sim_state.mode == GameMode::Human {
                    commands.spawn(AudioBundle {
                        source: asset_server.load("audio/point.ogg"),
                        settings: PlaybackSettings::DESPAWN,
                        ..default()
                    });
                }
            }
        }
    }
}
pub fn render_score(game: Res<Game>, mut query: Query<&mut TextureAtlas, With<ScoreText>>) {
    let score_string = format!("{:03}", game.score); // Ensure at least 3 digits, pad with zeros
    let score_digits: Vec<usize> = score_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for (digit, mut texture_atlas) in score_digits.iter().zip(query.iter_mut()) {
        texture_atlas.index = *digit;
    }
}

pub fn render_high_score(
    game: Res<Game>,
    mut query: Query<&mut TextureAtlas, With<HighScoreText>>,
) {
    // For high score, we'll display it differently, perhaps with an "HS" prefix
    // or at a different position. For now, let's just display it the same way as score
    // but we can modify this later if needed.
    let high_score_string = format!("{:03}", game.high_score); // Ensure at least 3 digits, pad with zeros
    let high_score_digits: Vec<usize> = high_score_string
        .chars()
        .map(|c| c.to_digit(10).unwrap() as usize)
        .collect();

    for (digit, mut texture_atlas) in high_score_digits.iter().zip(query.iter_mut()) {
        texture_atlas.index = *digit;
    }
}

pub fn reset_game_after_game_over(
    mut game: ResMut<Game>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut upper_pipe_query: Query<(&mut Transform, &mut UpperPipe), (With<UpperPipe>, Without<Bird>)>,
    mut lower_pipe_query: Query<
        &mut Transform,
        (With<LowerPipe>, Without<Bird>, Without<UpperPipe>),
    >,
    mut bird_query: Query<(&mut Bird, &mut Transform), Without<UpperPipe>>,
    mut space_query: Query<(&mut PressSpaceBarText, &mut Visibility)>,
    mut game_over_query: Query<&mut Visibility, (With<GameOverText>, Without<PressSpaceBarText>)>,
) {
    // Only process if game is over and space is pressed
    // Allow both Space and R key to reset
    if game.state != GameState::GameOver
        || !(keyboard_input.just_pressed(KeyCode::Space)
            || keyboard_input.just_pressed(KeyCode::KeyR))
    {
        return;
    }

    // Save high score if needed
    if game.score > game.high_score {
        game.high_score = game.score;
    }

    // Reset game state completely
    game.score = 0;
    game.state = GameState::Active;

    // Reset all pipes to starting positions
    for (i, (mut transform, mut upper_pipe)) in upper_pipe_query.iter_mut().enumerate() {
        upper_pipe.passed = false;
        transform.translation.x = 400.0 + (i as f32 * 300.0); // Adjust spacing as needed
                                                              // You might also want to randomize pipe heights here
    }

    for (i, mut transform) in lower_pipe_query.iter_mut().enumerate() {
        transform.translation.x = 400.0 + (i as f32 * 300.0);
    }

    // Reset bird to exact starting state
    for (mut bird, mut transform) in bird_query.iter_mut() {
        bird.velocity = 0.0;
        transform.translation.x = 100.0; // Reset X position too
        transform.translation.y = 0.0; // Or your BIRD_START_Y
        transform.rotation = Quat::IDENTITY;
    }

    // Hide all game over UI
    if let Ok((mut space_text, mut visibility)) = space_query.get_single_mut() {
        space_text.0.reset();
        *visibility = Visibility::Hidden;
    }

    if let Ok(mut game_over_visibility) = game_over_query.get_single_mut() {
        *game_over_visibility = Visibility::Hidden;
    }

    println!("Game reset! Starting fresh...");
}

pub fn bird_brain_system(
    mut bird_query: Query<(&mut Bird, &Transform)>,
    upper_pipe_query: Query<(&UpperPipe, &Transform)>,
    lower_pipe_query: Query<(&LowerPipe, &Transform)>,
    sim_state: Res<SimulationState>,
) {
    if sim_state.mode != GameMode::AI {
        return;
    }

    let bird_x = 0.0; // Birds are fixed at x=0 visually, but logically they are at 0

    let mut closest_pipe_dist = f32::MAX;
    let mut closest_upper_pipe: Option<&Transform> = None;
    let mut closest_lower_pipe: Option<&Transform> = None;

    // Find the next pipe
    for (_upper_pipe, upper_transform) in upper_pipe_query.iter() {
        // +26.0 accounts for bird/pipe width overlap roughly, ensuring we don't pick a pipe we are currently inside effectively "passed"
        // Adjust logic if needed: we want the pipe that is strictly in front or currently intersecting
        let dist = upper_transform.translation.x - bird_x + 52.0; // Pipe width is 52
        if dist > 0.0 && dist < closest_pipe_dist {
            closest_pipe_dist = dist;
            closest_upper_pipe = Some(upper_transform);
        }
    }
    
    // Match lower pipe
    if let Some(upper) = closest_upper_pipe {
         for (_, lower_transform) in lower_pipe_query.iter() {
             if (lower_transform.translation.x - upper.translation.x).abs() < 1.0 {
                 closest_lower_pipe = Some(lower_transform);
                 break;
             }
         }
    }

    for (mut bird, transform) in bird_query.iter_mut() {
        if bird.is_dead { continue; }
        
        bird.fitness += 1.0;

        if let (Some(upper), Some(lower)) = (closest_upper_pipe, closest_lower_pipe) {
                 //  inputs
                 // 1. Bird Y (normalized)
                 // 2. Dist to Gap Center Y (normalized)
                 // 3. Dist to Pipe X (normalized)
                 // 4. Velocity
                
                 let gap_center_y = (upper.translation.y + lower.translation.y) / 2.0;
                 let bird_y = transform.translation.y;
                 let dist_to_pipe_x = upper.translation.x; // bird is at 0
                 
                 let inputs = vec![
                     map_range(bird_y as f64, -300.0, 300.0, 0.0, 1.0).clamp(0.0, 1.0),
                     map_range(gap_center_y as f64, -300.0, 300.0, 0.0, 1.0).clamp(0.0, 1.0),
                     // Expand range to -50.0 to account for when bird is crossing the pipe
                     map_range(dist_to_pipe_x as f64, -50.0, 500.0, 0.0, 1.0).clamp(0.0, 1.0),
                     // Expand range to -1000.0 to capture terminal velocity (falling fast)
                     map_range(bird.velocity as f64, -1000.0, 500.0, 0.0, 1.0).clamp(0.0, 1.0),
                     // Extra input: difference
                     map_range((bird_y - gap_center_y) as f64, -300.0, 300.0, 0.0, 1.0).clamp(0.0, 1.0),
                 ];
                 
                 // Precision Reward: Reward staying close to the center of the gap
                 // Only apply when close to the pipe to encourage alignment
                 let mut precision_bonus = 0.0;
                 if dist_to_pipe_x < 100.0 && dist_to_pipe_x > -50.0 {
                    let vertical_dist = (bird_y - gap_center_y).abs();
                    // Bonus is higher when vertical_dist is small. 
                    // Max bonus 0.5 per frame roughly
                    if vertical_dist < 50.0 {
                        precision_bonus = (50.0 - vertical_dist) / 50.0;
                    }
                 }
                 
                 let should_jump = if let Some(brain) = &bird.brain {
                     brain.predict(&inputs)[0] > 0.5
                 } else {
                     false
                 };

                 bird.fitness += precision_bonus;
                 
                 if should_jump {
                     bird.velocity = 300.0;
                 }
        }
    }
}

fn map_range(val: f64, in_min: f64, in_max: f64, out_min: f64, out_max: f64) -> f64 {
    (val - in_min) * (out_max - out_min) / (in_max - in_min) + out_min
}

pub fn check_alive_and_next_gen(
    mut bird_query: Query<(&mut Bird, &mut Transform)>,
    mut upper_pipe_query: Query<(&mut Transform, &mut UpperPipe), (With<UpperPipe>, Without<Bird>)>,
    mut lower_pipe_query: Query<&mut Transform, (With<LowerPipe>, Without<Bird>, Without<UpperPipe>)>,
    mut sim_state: ResMut<SimulationState>,
    mut game: ResMut<Game>,
) {
    if sim_state.mode != GameMode::AI {
        return;
    }

    let alive_count = bird_query.iter().filter(|(b, _)| !b.is_dead).count();
    sim_state.birds_alive = alive_count;

    if alive_count == 0 {
        sim_state.generation += 1;
        
        // Collect all birds
        let mut birds: Vec<(Net, f32)> = bird_query.iter()
            .map(|(b, _)| (b.brain.clone().unwrap(), b.fitness))
            .collect();
            
        // Sort by fitness descending
        birds.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        
        println!("Evolving gen {} -> {}. Max Fitness: {}", sim_state.generation - 1, sim_state.generation, birds.first().map(|b| b.1).unwrap_or(0.0));

        let mut new_brains = Vec::new();
        
        // Elitism: Keep top 4 best performing brains EXACTLY as they are
        for i in 0..4 {
            if i < birds.len() {
                new_brains.push(birds[i].0.clone());
            }
        }
        
        // Generate the rest
        let mut rng = thread_rng();
        let remaining_slots = NUM_BIRDS - new_brains.len();
        
        // Selection: weighted by fitness, but favor top 50% significantly if possible?
        // Standard weighted index is fine if weights are distinct enough.
        // Let's square the fitness to exaggerate differences
        let weights: Vec<f32> = birds.iter().map(|(_, f)| f.powf(2.0)).collect();

        if let Ok(dist) = WeightedIndex::new(weights) {
             for _ in 0..remaining_slots {
                let parent_idx = dist.sample(&mut rng);
                let mut child_brain = birds[parent_idx].0.clone();
                child_brain.mutate();
                new_brains.push(child_brain);
            }
        } else {
             // Fallback if all 0 fitness (shouldn't happen usually)
             for _ in 0..remaining_slots {
                 let mut child_brain = birds[0].0.clone(); // Just clone the first
                 child_brain.mutate();
                 new_brains.push(child_brain);
             }
        }

        // Assign new brains
        // IMPORTANT: We must re-assign based on index or just refill. 
        // Bevy query iteration order isn't guaranteed relative to our vector if we didn't track entities.
        // But since we are replacing all of them, we can just zip.
        
        for ((mut bird, mut transform), new_brain) in bird_query.iter_mut().zip(new_brains.into_iter()) {
            bird.is_dead = false;
            bird.velocity = 0.0;
            bird.fitness = 0.0;
            bird.brain = Some(new_brain);
            transform.translation.y = 0.0;
            transform.translation.x = 0.0; 
            transform.rotation = Quat::IDENTITY;
        }

        // 4. Reset pipes
        // Match upper and lower pipes by index to ensure they get the same random position
        let mut upper_iter = upper_pipe_query.iter_mut();
        let mut lower_iter = lower_pipe_query.iter_mut();
        
        let mut i = 0;
        while let (Some((mut upper_transform, mut upper_pipe)), Some(mut lower_transform)) = (upper_iter.next(), lower_iter.next()) {
             upper_pipe.passed = false;
             let delta_x = i as f32 * 200.0 + 400.0;
             
             let (lower_y, upper_y) = random_pipe_position();
             
             upper_transform.translation.x = delta_x;
             upper_transform.translation.y = upper_y;
             
             lower_transform.translation.x = delta_x;
             lower_transform.translation.y = lower_y;
             
             i += 1;
        }
        
        game.score = 0;
    }
}

pub fn update_gen_ui(
    sim_state: Res<SimulationState>,
    mut query: Query<&mut Text, With<GenUi>>,
) {
    for mut text in query.iter_mut() {
        if sim_state.mode == GameMode::AI {
            text.sections[0].value = format!("Gen: {}\nAlive: {}", sim_state.generation, sim_state.birds_alive);
            text.sections[0].style.color = Color::WHITE;
        } else {
             text.sections[0].value = "".to_string(); 
        }
    }
}

pub fn toggle_game_mode(
    mut sim_state: ResMut<SimulationState>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    bird_query: Query<Entity, With<Bird>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut game: ResMut<Game>,
    mut upper_pipes: Query<&mut Transform, With<UpperPipe>>,
    mut lower_pipes: Query<&mut Transform, (With<LowerPipe>, Without<UpperPipe>)>,
) {
     if keyboard_input.just_pressed(KeyCode::KeyM) {
         for entity in bird_query.iter() {
             commands.entity(entity).despawn();
         }

         let bird_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
            UVec2::new(34, 24),
            3,
            1,
            None,
            None,
        ));

         if sim_state.mode == GameMode::AI {
             sim_state.mode = GameMode::Human;
              commands.spawn((
                SpriteBundle {
                    texture: asset_server.load("texture/bird.png"),
                    transform: Transform::from_xyz(0., 0., 2.),
                    ..default()
                },
                TextureAtlas {
                    index: 1,
                    layout: bird_layout,
                },
                 Bird {
                    timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                    velocity: 0.,
                    brain: None, 
                    is_dead: false,
                    fitness: 0.0,
                },
            ));
            game.score = 0;
            // Reset pipes for human mode
            let mut upper_iter = upper_pipes.iter_mut();
            let mut lower_iter = lower_pipes.iter_mut();
            let mut i = 0;
            while let (Some(mut upper_transform), Some(mut lower_transform)) = (upper_iter.next(), lower_iter.next()) {
                 let delta_x = i as f32 * 200.0 + 400.0;
                 let (lower_y, upper_y) = random_pipe_position();
                 
                 upper_transform.translation.x = delta_x;
                 upper_transform.translation.y = upper_y;
                 
                 lower_transform.translation.x = delta_x;
                 lower_transform.translation.y = lower_y;
                 i += 1;
            }

         } else {
             sim_state.mode = GameMode::AI;
             sim_state.generation = 1;
             
             for _ in 0..NUM_BIRDS {
                commands.spawn((
                    SpriteBundle {
                        texture: asset_server.load("texture/bird.png"),
                        transform: Transform::from_xyz(0., 0., 2.),
                        ..default()
                    },
                    TextureAtlas {
                        index: 1,
                        layout: bird_layout.clone(),
                    },
                     Bird {
                        timer: Timer::from_seconds(0.2, TimerMode::Repeating),
                        velocity: 0.,
                        brain: Some(Net::new(vec![5, 8, 1])), 
                        is_dead: false,
                        fitness: 0.0,
                    },
                ));
            }
         }
         
          game.state = GameState::Active; 
          game.score = 0;
     }
}
