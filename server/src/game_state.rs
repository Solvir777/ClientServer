use std::collections::HashMap;
use nalgebra::Vector2;
use rand::random;
use common::game_state::{Food, GameStateUpdate, Player, PlayerState};
use common::game_state::PlayerState::Alive;
use common::UserId;

pub struct ServerGameState {
    pub(crate) user_id_to_index: HashMap<UserId, usize>,
    pub(crate) players: Vec<PlayerState>,
    pub(crate) foods: Vec<Food>,
    board_size: Vector2<f32>,
}



impl ServerGameState {

    pub fn to_player(&self, id: &UserId) -> GameStateUpdate {
        let index = self.user_id_to_index.get(id).unwrap();
        GameStateUpdate {
            you: *index as u8,
            players: self.players.clone(),
            foods: self.foods.clone(),
        }
    }
    pub fn new() -> ServerGameState {
        Self{
            user_id_to_index: Default::default(),
            players: Default::default(),
            foods: Default::default(),
            board_size: Vector2::new(200., 200.),
        }
    }
    /// This function guarantees that id will be a living player
    pub fn spawn_player(&mut self, id: UserId) {
        let state = Alive(Player{position: self.random_position(), size: 20});
        if let Some(&index) = self.user_id_to_index.get(&id){
            if let PlayerState::Dead = self.players[index] {
                self.players[index] = state;
            }
            return;
        }
        self.user_id_to_index.insert(id, self.players.len());
        self.players.push(state);
    }

    pub fn move_player(&mut self, id: UserId, new_pos: Vector2<f32>) {
        if let Some(&index) = self.user_id_to_index.get(&id) && let Alive(player) = &mut self.players[index] {
            player.position = new_pos;
        }else {
            println!("received move request for dead or unrecognized player");
        }
    }

    pub fn spawn_food(&mut self){
        let pos = self.random_position();
        self.foods.push(Food{position: pos});
    }
    fn random_position(&self) -> Vector2<f32>{
        Vector2::new(random(), random::<f32>()).component_mul(&self.board_size)
    }

    pub fn tick(&mut self) {
        println!("players: {:?}", self.players);
        //chance to spawn food
        if random::<f32>() > 0.95 {
            self.spawn_food();
        }

        let mut alive_players = self.players.iter_mut()
            .filter_map(|state| if let Alive(_) = state {Some(state)} else {None}).collect::<Vec<_>>();
        if alive_players.is_empty() {
            return;
        }

        for i in 0..alive_players.len() {
            for j in (i+1)..alive_players.len() {
                try_eat_each_other(&mut alive_players, i, j);
            }
            try_eat_food(alive_players[i], &mut self.foods);
        }
    }
}

fn try_eat_food(state: &mut PlayerState, foods: &mut Vec<Food>) {
    if let Alive(player) = state {
        foods.retain(
            |food| {
                if food.position.metric_distance(&player.position) < player.size as f32 {
                    player.size += 1;
                    return false;
                }
                true
            }
        );
    }
}

fn try_eat_each_other(players: &mut Vec<&mut PlayerState>, i1: usize, i2: usize) {
    let [a_state, b_state] = players.get_disjoint_mut([i1, i2]).unwrap();
    if let Alive(a) = a_state && let Alive(b) = b_state {
        let diff = a.position.metric_distance(&b.position);
        if a.size > b.size && a.size as f32 > diff {
            a.size += b.size;
            **b_state = PlayerState::Dead;
        }
        else if a.size as f32 > diff {
            b.size += a.size;
            **a_state = PlayerState::Dead;
        }
    }
}
