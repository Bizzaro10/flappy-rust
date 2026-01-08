use bevy::prelude::*;
#[derive(Resource, Default)]
pub struct Game {
    pub score: u32,
    pub high_score: u32,
    pub state: GameState,
}
 #[derive(PartialEq)]
pub enum GameState {
    Active,
    Inactive,
    GameOver,
}
 
impl Default for GameState {
    fn default() -> Self {
        GameState::Inactive
    }
}

pub fn is_game_active(game: Res<Game>) -> bool {
    game.state == GameState::Active
}
 
pub fn is_game_not_active(game: Res<Game>) -> bool {
    game.state != GameState::Active
}
pub fn is_game_over(game: Res<Game>) -> bool {
    game.state == GameState::GameOver
}

#[derive(Resource)]
pub struct SimulationState {
    pub generation: u32,
    pub birds_alive: usize,
    pub mode: GameMode,
}

impl Default for SimulationState {
    fn default() -> Self {
        Self {
            generation: 1,
            birds_alive: 1000,
            mode: GameMode::AI,
        }
    }
}

#[derive(Default, PartialEq, Clone, Copy, Debug)]
pub enum GameMode {
    Human,
    #[default]
    AI,
}

#[allow(dead_code)]
pub fn is_ai_mode(sim_state: Res<SimulationState>) -> bool {
    sim_state.mode == GameMode::AI
}

#[allow(dead_code)]
pub fn is_human_mode(sim_state: Res<SimulationState>) -> bool {
    sim_state.mode == GameMode::Human
}