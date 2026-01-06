use nalgebra::Vector2;
use serializeable::Serializeable;
use crate::UserId;

#[derive(Serializeable, Debug)]
pub enum ClientConnectionMessage{
    ConnectNew,
    ConnectWithId(UserId),
}

#[derive(Serializeable, Debug)]
pub enum ClientTcpMessage {
    Respawn,
}


#[derive(Serializeable, Debug)]
pub enum ClientUdpMessage {
    Move{new_pos: Vector2<f32>},
}
