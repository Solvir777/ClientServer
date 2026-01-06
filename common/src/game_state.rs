use nalgebra::Vector2;
use serializeable::Serializeable;

#[derive(Serializeable, Debug)]
pub struct GameStateUpdate{
    ///index into players
    pub you: u8,
    pub players: Vec<PlayerState>,
    pub foods: Vec<Food>,
}
#[derive(Serializeable, Debug, Clone)]
pub enum PlayerState{
    Alive(Player),
    Dead,
}

#[derive(Serializeable, Debug, Clone)]
pub struct Food{
    pub position: Vector2<f32>,
}

#[derive(Serializeable, Debug, Clone)]
pub struct Player{
    pub position: Vector2<f32>,
    pub size: u16,
}

