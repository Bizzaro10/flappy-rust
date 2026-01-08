use bevy::prelude::*;
use crate::constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use resources::*;
use setup::setup;
use systems::*;

mod components;
mod constants;
mod resources;
mod setup;
mod systems;
mod utils;
mod nn;
mod ui;

use bevy_egui::EguiPlugin;
use ui::{UiState, ui_system};

fn main() {
    App::new()
        .init_resource::<Game>()
        .init_resource::<SimulationState>()
        .init_resource::<UiState>()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Flappy Rust".to_string(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(EguiPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, blink_space_bar_text.run_if(is_game_not_active))
        .add_systems(Update, move_background.run_if(is_game_active))
        .add_systems(Update, move_ground.run_if(is_game_active))
        .add_systems(Update, animate_bird.run_if(is_game_active))
        .add_systems(Update, start_game.run_if(is_game_not_active))
        .add_systems(Update, gravity.run_if(is_game_active))
        .add_systems(Update, jump.run_if(is_game_active))
        .add_systems(Update, pipes.run_if(is_game_active))
        .add_systems(Update, score.run_if(is_game_active))
        .add_systems(Update, render_score.run_if(is_game_active))
        .add_systems(Update, render_high_score.run_if(is_game_not_active))
        .add_systems(Update, reset_game_after_game_over.run_if(is_game_over))
        // AI Systems
        .add_systems(Update, bird_brain_system.run_if(is_game_active))
        .add_systems(Update, check_alive_and_next_gen.run_if(is_game_active))
        .add_systems(Update, update_gen_ui)
        .add_systems(Update, toggle_game_mode)
        .add_systems(Update, ui_system)
        .run();
}
