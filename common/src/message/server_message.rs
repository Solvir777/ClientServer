use serializeable::Serializeable;
use crate::game_state::GameStateUpdate;
use crate::UserId;


/// Used when a client is connecting
#[derive(Serializeable, Debug)]
pub enum ServerConnectionMessage{
    AssignUserId(UserId),
    AcknowledgeId,
    IdAlreadyInUse,
}


#[derive(Serializeable, Debug)]
pub enum ServerUdpMessage {
    GameState(GameStateUpdate)
}

#[derive(Serializeable, Debug)]
pub enum ServerTcpMessage {
    Nothing,
}