use bevy::prelude::*;
use plugin::MyPlugin;
use resources::*;
use setup::setup;
use systems::*;
use ai_resources::*;
use ai_systems::*;

mod components;
mod constants;
mod plugin;
mod resources;
mod setup;
mod systems;
mod utils;
mod ai;
mod ai_components;
mod ai_resources;
mod ai_systems;
 

 

 
fn main() {
    App::new()
        .init_resource::<Game>()
        .init_resource::<AITraining>()
        .init_resource::<AIStats>()
        .add_systems(Startup, setup)
        .add_systems(Startup, setup_ai_ui)
        .add_systems(Update, blink_space_bar_text.run_if(is_game_not_active))
        .add_systems(Update, move_background.run_if(is_game_active))
        .add_systems(Update, move_ground.run_if(is_game_active))
        .add_systems(Update, animate_bird.run_if(is_game_active))
        .add_systems(Update, start_game.run_if(is_game_not_active))
        .add_systems(Update, spawn_ai_population.run_if(is_game_active))
        .add_systems(Update, ai_bird_thinking.run_if(is_game_active))
        .add_systems(Update, ai_bird_physics.run_if(is_game_active))
        .add_systems(Update, ai_bird_collision.run_if(is_game_active))
        .add_systems(Update, ai_bird_scoring.run_if(is_game_active))
        .add_systems(Update, ai_bird_animation.run_if(is_game_active))
        .add_systems(Update, ai_generation_management.run_if(is_game_active))
        .add_systems(Update, update_ai_ui)
        .add_systems(Update, gravity.run_if(is_game_active))
        .add_systems(Update, jump.run_if(is_game_active))
        .add_systems(Update, pipes.run_if(is_game_active))
        .add_systems(Update, score.run_if(is_game_active))
        .add_systems(Update, render_score.run_if(is_game_active))
        .add_systems(Update, render_high_score.run_if(is_game_not_active))
        .add_systems(Update, reset_game_after_game_over.run_if(is_game_over))
        .add_plugins(MyPlugin)
        .run();
}