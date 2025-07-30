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