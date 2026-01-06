use common::game_state::{Food, GameStateUpdate, PlayerState};

pub struct ClientGameState {
    pub player: PlayerState,
    pub other_players: Vec<PlayerState>,
    pub foods: Vec<Food>,
}


impl ClientGameState {
    pub fn new() -> Self {
        Self{
            player: PlayerState::Dead,
            other_players: vec!(),
            foods: vec!(),
        }
    }

    pub fn update_with_state(&mut self, mut update: GameStateUpdate) {
        let this = update.players.remove(update.you as usize);
        *self = Self{
            player: this,
            other_players: update.players,
            foods: update.foods,
        };
    }
}