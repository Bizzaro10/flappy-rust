use bevy::prelude::*;
use crate::{
    components::*,
    // components::PressSpaceBarText,
    constants::{WINDOW_HEIGHT, WINDOW_WIDTH},
    utils::random_pipe_position,
};
use crate::nn::Net;
use crate::resources::{SimulationState, GameMode};
use crate::constants::NUM_BIRDS;

pub fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
    sim_state: Res<SimulationState>,
) {
    // Spawn a 2D camera
    commands.spawn(Camera2dBundle::default());

    // Spawn the background
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/background.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH + 288.0 * 2., WINDOW_HEIGHT)), // Adding a custom size
                ..default() // Everything else is set to default
            },
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,  // Only repeat on the x-axis
            tile_y: false, // no repeat on the y-axis
            stretch_value: 1., // no stretching
        },
        Background,
    ));
    // Spawn the Ground
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/base.png"),
            sprite: Sprite {
                custom_size: Some(Vec2::new(WINDOW_WIDTH + 288. * 2., 112.)),
                ..default()
            },
            transform: Transform::from_xyz(0., -250., 1.),
            ..default()
        },
        ImageScaleMode::Tiled {
            tile_x: true,
            tile_y: false,
            stretch_value: 1.,
        },
        Ground,
    ));
    // Game Over Text
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/game-over.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            visibility: Visibility::Hidden,
            ..default()
        },
        GameOverText,
    ));
    // Space Bar Text
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/space.png"),
            transform: Transform::from_xyz(0.0, -50.0, 1.0),
            ..default()
        },
        PressSpaceBarText(Timer::from_seconds(0.5, TimerMode::Repeating)),
    ));

    let number_layout: TextureAtlasLayout =
        TextureAtlasLayout::from_grid(UVec2::new(24, 36), 1, 10, None, None);
    let number_texture_atlas_layout: Handle<TextureAtlasLayout> =
        texture_atlas_layouts.add(number_layout);

    // Create three score digits
    let digit_positions = [-350.0, -320.0, -290.0]; // Positions for each digit
    for (_i, &x_pos) in digit_positions.iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/numbers.png"),
                transform: Transform::from_xyz(x_pos, 200.0, 1.0),
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: number_texture_atlas_layout.clone(),
            },
            ScoreText,
        ));
    }

    // Create three high score digits (displayed below the regular score)
    let high_score_digit_positions = [-350.0, -320.0, -290.0]; // Same x positions as regular score
    for (_i, &x_pos) in high_score_digit_positions.iter().enumerate() {
        commands.spawn((
            SpriteBundle {
                texture: asset_server.load("texture/numbers.png"),
                transform: Transform::from_xyz(x_pos, 150.0, 1.0), // Different y position
                ..default()
            },
            TextureAtlas {
                index: 0,
                layout: number_texture_atlas_layout.clone(),
            },
            HighScoreText,
        ));
    }

    // Gen UI
    commands.spawn((
        TextBundle::from_section(
            "Gen: 1\nAlive: 0",
            TextStyle {
                font: asset_server.load("fonts/FlappyBird.ttf"), // Assuming font exists or default
                font_size: 30.0,
                color: Color::WHITE,
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
        GenUi,
    ));


    let bird_layout = texture_atlas_layouts.add(TextureAtlasLayout::from_grid(
        UVec2::new(34, 24),
        3,
        1,
        None,
        None,
    ));

    let num_birds = if sim_state.mode == GameMode::AI { NUM_BIRDS } else { 1 };
    
    for _ in 0..num_birds {
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
                brain: if sim_state.mode == GameMode::AI { Some(Net::new(vec![5, 8, 1])) } else { None },
                is_dead: false,
                fitness: 0.0,
            },
        ));
    }
//     // Spawn Lower Pipe
// commands.spawn((
//     SpriteBundle {
//         texture: asset_server.load("texture/pipe.png"),
//         transform: Transform::from_xyz(350., -200., 0.5),
//         ..default()
//     },
//     LowerPipe,
// ));
// let mut transform = Transform::from_xyz(350., 250., 0.5);
// transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));
 
//  // Spawn Upper Pipe
// commands.spawn((
//     SpriteBundle {
//         texture: asset_server.load("texture/pipe.png"),
//         transform,
//         ..default()
//     },
//     UpperPipe,
// ));
// for i in 0..5 {
//     let delta_x = i as f32 * 200.; // Space between pairs of pipes
//     let mut transform = Transform::from_xyz(350. + delta_x, -250., 0.5);
 
//     // Spawn Lower Pipe
//     commands.spawn((
//         SpriteBundle {
//             texture: asset_server.load("texture/pipe.png"),
//             transform,
//             ..default()
//         },
//         LowerPipe,
//     ));
 
//     // Rotating the upper pipe
//     transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));
//     // Changing the y position of the upper pipe
//     transform.translation.y += 450.;
 
//     // Spawn Upper Pipe
//     commands.spawn((
//         SpriteBundle {
//             texture: asset_server.load("texture/pipe.png"),
//             transform,
//             ..default()
//         },
//         UpperPipe,
//     ));
//   }
  for i in 0..5 {
    let delta_x = i as f32 * 200.;
    let (lower_y, upper_y) = random_pipe_position();
    let mut transform = Transform::from_xyz(350. + delta_x, lower_y, 0.5);
 
    // Spawn Lower Pipe
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/pipe.png"),
            transform,
            ..default()
        },
        LowerPipe,
    ));
 
    transform.rotate(Quat::from_rotation_z(std::f32::consts::PI));
    transform.translation.y = upper_y;
 
    // Spawn Upper Pipe
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("texture/pipe.png"),
            transform,
            ..default()
        },
        UpperPipe{passed:false},
    ));
}
   
}
