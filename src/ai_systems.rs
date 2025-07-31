use bevy::prelude::*;
use crate::{
    ai::AIBird,
    ai_components::*,
    ai_resources::*,
    components::*,
    resources::*,
    constants::*,
};

pub fn setup_ai_ui(mut commands: Commands, asset_server: Res<AssetServer>) {
    // AI Stats UI
    commands.spawn((
        TextBundle::from_section(
            "Generation: 1",
            TextStyle {
                font: default(),
                font_size: 20.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        GenerationText,
    ));
    
    commands.spawn((
        TextBundle::from_section(
            "Population: 50",
            TextStyle {
                font: default(),
                font_size: 16.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(35.0),
            left: Val::Px(10.0),
            ..default()
        }),
        PopulationText,
    ));
    
    commands.spawn((
        TextBundle::from_section(
            "Alive: 50",
            TextStyle {
                font: default(),
                font_size: 16.0,
                color: Color::GREEN,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(55.0),
            left: Val::Px(10.0),
            ..default()
        }),
        AliveCountText,
    ));
    
    commands.spawn((
        TextBundle::from_section(
            "Best Fitness: 0.0",
            TextStyle {
                font: default(),
                font_size: 16.0,
                color: Color::YELLOW,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(75.0),
            left: Val::Px(10.0),
            ..default()
        }),
        BestFitnessText,
    ));
    
    commands.spawn((
        TextBundle::from_section(
            "Avg Fitness: 0.0",
            TextStyle {
                font: default(),
                font_size: 16.0,
                color: Color::CYAN,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(95.0),
            left: Val::Px(10.0),
            ..default()
        }),
        AverageFitnessText,
    ));
}

pub fn spawn_ai_population(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    mut ai_training: ResMut<AITraining>,
) {
    if ai_training.training_active {
        return;
    }
    
    ai_training.training_active = true;
    ai_training.alive_count = ai_training.current_population.len();
    ai_training.generation_timer.reset();
    
    let texture_atlas_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(34, 24),
        3,
        1,
        None,
        None,
    ));
    
    // Spawn AI birds
    for (i, brain) in ai_training.current_population.iter().enumerate() {
        let x_offset = (i as f32 % 10.0) * 5.0 - 22.5; // Spread birds horizontally
        let y_offset = (i as f32 / 10.0).floor() * 5.0 - 10.0; // Stack vertically
        
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/bird.png"),
                transform: Transform::from_xyz(x_offset, y_offset, 2.0),
                ..default()
            },
            TextureAtlas {
                index: 1,
                layout: texture_atlas_layout.clone(),
            },
            AIBird::new(brain.clone()),
            AIBirdMarker,
        ));
    }
}

pub fn ai_bird_thinking(
    mut ai_bird_query: Query<(&mut AIBird, &Transform), With<AIBirdMarker>>,
    upper_pipe_query: Query<&Transform, (With<UpperPipe>, Without<AIBirdMarker>)>,
    lower_pipe_query: Query<&Transform, (With<LowerPipe>, Without<AIBirdMarker>, Without<UpperPipe>)>,
) {
    // Find the closest pipe
    let mut closest_upper_pipe: Option<&Transform> = None;
    let mut closest_lower_pipe: Option<&Transform> = None;
    let mut closest_distance = f32::INFINITY;
    
    for upper_transform in upper_pipe_query.iter() {
        if upper_transform.translation.x > -50.0 && upper_transform.translation.x < closest_distance {
            closest_distance = upper_transform.translation.x;
            closest_upper_pipe = Some(upper_transform);
        }
    }
    
    for lower_transform in lower_pipe_query.iter() {
        if lower_transform.translation.x > -50.0 && lower_transform.translation.x == closest_distance {
            closest_lower_pipe = Some(lower_transform);
        }
    }
    
    for (mut ai_bird, bird_transform) in ai_bird_query.iter_mut() {
        if !ai_bird.alive {
            continue;
        }
        
        ai_bird.update_frames();
        
        // Prepare inputs for neural network
        let mut inputs = vec![0.0; 4];
        
        // Input 1: Bird's y position (normalized)
        inputs[0] = bird_transform.translation.y / WINDOW_HEIGHT;
        
        // Input 2: Bird's velocity (normalized)
        inputs[1] = ai_bird.velocity / 600.0;
        
        if let (Some(upper_pipe), Some(lower_pipe)) = (closest_upper_pipe, closest_lower_pipe) {
            // Input 3: Distance to next pipe (normalized)
            inputs[2] = (upper_pipe.translation.x - bird_transform.translation.x) / WINDOW_WIDTH;
            
            // Input 4: Gap center y position (normalized)
            let gap_center = (upper_pipe.translation.y + lower_pipe.translation.y) / 2.0;
            inputs[3] = gap_center / WINDOW_HEIGHT;
        }
        
        // Let the AI decide whether to jump
        let should_jump = ai_bird.think(&inputs);
        
        if should_jump {
            ai_bird.velocity = 300.0;
        }
    }
}

pub fn ai_bird_physics(
    time: Res<Time>,
    mut ai_bird_query: Query<(&mut AIBird, &mut Transform), With<AIBirdMarker>>,
    mut ai_training: ResMut<AITraining>,
) {
    let delta = time.delta().as_secs_f32();
    let gravity = 9.8;
    let delta_v = gravity * 150.0 * delta;
    
    let mut alive_count = 0;
    
    for (mut ai_bird, mut transform) in ai_bird_query.iter_mut() {
        if !ai_bird.alive {
            continue;
        }
        
        alive_count += 1;
        
        // Apply gravity
        ai_bird.velocity -= delta_v;
        transform.translation.y += ai_bird.velocity * delta;
        
        // Rotate the bird
        let rotation = ai_bird.velocity / 600.0;
        let max_rotation = 0.5;
        transform.rotation = Quat::from_rotation_z(rotation.max(-max_rotation).min(max_rotation));
        
        // Check ground collision
        let ground_y = -250.0;
        let ground_height = 112.0;
        let bird_height = 23.0;
        let collision_point = ground_y + ground_height / 2.0 + bird_height / 2.0;
        
        if transform.translation.y < collision_point {
            transform.translation.y = collision_point;
            ai_bird.die();
        }
        
        // Check ceiling collision
        if transform.translation.y > 260.0 {
            transform.translation.y = 260.0;
            ai_bird.die();
        }
    }
    
    ai_training.alive_count = alive_count;
}

pub fn ai_bird_collision(
    mut ai_bird_query: Query<(&mut AIBird, &Transform), With<AIBirdMarker>>,
    upper_pipe_query: Query<&Transform, (With<UpperPipe>, Without<AIBirdMarker>)>,
    lower_pipe_query: Query<&Transform, (With<LowerPipe>, Without<AIBirdMarker>, Without<UpperPipe>)>,
) {
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
    
    for (mut ai_bird, bird_transform) in ai_bird_query.iter_mut() {
        if !ai_bird.alive {
            continue;
        }
        
        // Check collision with upper pipes
        for pipe_transform in upper_pipe_query.iter() {
            if is_collision(bird_transform, pipe_transform) {
                ai_bird.die();
                break;
            }
        }
        
        // Check collision with lower pipes
        if ai_bird.alive {
            for pipe_transform in lower_pipe_query.iter() {
                if is_collision(bird_transform, pipe_transform) {
                    ai_bird.die();
                    break;
                }
            }
        }
    }
}

pub fn ai_bird_scoring(
    mut ai_bird_query: Query<(&mut AIBird, &Transform), With<AIBirdMarker>>,
    mut upper_pipe_query: Query<(&mut UpperPipe, &Transform)>,
) {
    for (mut ai_bird, bird_transform) in ai_bird_query.iter_mut() {
        if !ai_bird.alive {
            continue;
        }
        
        for (mut upper_pipe, pipe_transform) in upper_pipe_query.iter_mut() {
            let passed = pipe_transform.translation.x < bird_transform.translation.x;
            
            if passed && !upper_pipe.passed {
                ai_bird.score += 1;
                upper_pipe.passed = true;
            }
        }
    }
}

pub fn ai_generation_management(
    time: Res<Time>,
    mut ai_training: ResMut<AITraining>,
    mut ai_stats: ResMut<AIStats>,
    ai_bird_query: Query<&AIBird, With<AIBirdMarker>>,
    mut commands: Commands,
    ai_bird_entities: Query<Entity, With<AIBirdMarker>>,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    if !ai_training.training_active {
        return;
    }
    
    ai_training.generation_timer.tick(time.delta());
    
    // Check if generation should end (all dead or time limit reached)
    let all_dead = ai_bird_query.iter().all(|bird| !bird.alive);
    let time_up = ai_training.generation_timer.finished();
    
    if all_dead || time_up {
        // Collect fitness data
        let mut population_with_fitness = Vec::new();
        
        for (i, ai_bird) in ai_bird_query.iter().enumerate() {
            let fitness = if ai_bird.alive {
                // Calculate fitness for still-alive birds
                let mut bird_copy = ai_bird.clone();
                bird_copy.calculate_fitness();
                bird_copy.fitness
            } else {
                ai_bird.fitness
            };
            
            if i < ai_training.current_population.len() {
                population_with_fitness.push((ai_training.current_population[i].clone(), fitness));
            }
        }
        
        // Update stats
        ai_stats.best_fitness = ai_training.genetic_algorithm.get_best_fitness(&population_with_fitness);
        ai_stats.average_fitness = ai_training.genetic_algorithm.get_average_fitness(&population_with_fitness);
        ai_stats.generation = ai_training.genetic_algorithm.generation + 1;
        
        // Find best score
        ai_stats.best_score = ai_bird_query.iter()
            .map(|bird| bird.score)
            .max()
            .unwrap_or(0);
        
        // Evolve to next generation
        ai_training.current_population = ai_training.genetic_algorithm.evolve(&population_with_fitness);
        
        // Clean up current generation
        for entity in ai_bird_entities.iter() {
            commands.entity(entity).despawn();
        }
        
        // Reset for next generation
        ai_training.training_active = false;
        ai_training.generation_timer.reset();
        
        println!("Generation {} completed! Best fitness: {:.2}, Average fitness: {:.2}, Best score: {}", 
                 ai_stats.generation - 1, ai_stats.best_fitness, ai_stats.average_fitness, ai_stats.best_score);
    }
}

pub fn update_ai_ui(
    ai_stats: Res<AIStats>,
    ai_training: Res<AITraining>,
    mut generation_query: Query<&mut Text, (With<GenerationText>, Without<PopulationText>, Without<AliveCountText>, Without<BestFitnessText>, Without<AverageFitnessText>)>,
    mut population_query: Query<&mut Text, (With<PopulationText>, Without<GenerationText>, Without<AliveCountText>, Without<BestFitnessText>, Without<AverageFitnessText>)>,
    mut alive_query: Query<&mut Text, (With<AliveCountText>, Without<GenerationText>, Without<PopulationText>, Without<BestFitnessText>, Without<AverageFitnessText>)>,
    mut best_fitness_query: Query<&mut Text, (With<BestFitnessText>, Without<GenerationText>, Without<PopulationText>, Without<AliveCountText>, Without<AverageFitnessText>)>,
    mut avg_fitness_query: Query<&mut Text, (With<AverageFitnessText>, Without<GenerationText>, Without<PopulationText>, Without<AliveCountText>, Without<BestFitnessText>)>,
) {
    if let Ok(mut text) = generation_query.get_single_mut() {
        text.sections[0].value = format!("Generation: {}", ai_stats.generation);
    }
    
    if let Ok(mut text) = population_query.get_single_mut() {
        text.sections[0].value = format!("Population: {}", ai_training.current_population.len());
    }
    
    if let Ok(mut text) = alive_query.get_single_mut() {
        text.sections[0].value = format!("Alive: {}", ai_training.alive_count);
        text.sections[0].style.color = if ai_training.alive_count > 0 { Color::GREEN } else { Color::RED };
    }
    
    if let Ok(mut text) = best_fitness_query.get_single_mut() {
        text.sections[0].value = format!("Best Fitness: {:.1}", ai_stats.best_fitness);
    }
    
    if let Ok(mut text) = avg_fitness_query.get_single_mut() {
        text.sections[0].value = format!("Avg Fitness: {:.1}", ai_stats.average_fitness);
    }
}

pub fn ai_bird_animation(
    time: Res<Time>,
    mut ai_bird_query: Query<(&mut AIBird, &mut TextureAtlas), With<AIBirdMarker>>,
) {
    for (mut ai_bird, mut texture_atlas) in ai_bird_query.iter_mut() {
        if !ai_bird.alive {
            continue;
        }
        
        ai_bird.timer.tick(time.delta());
        
        if ai_bird.timer.finished() {
            texture_atlas.index = if texture_atlas.index == 2 {
                0
            } else {
                texture_atlas.index + 1
            };
        }
    }
}