use serializeable::Serializeable;
use crate::UserId;

#[derive(Serializeable, Debug)]
pub enum ServerTcpMessage {
    Text(String),
    AssignUserId(UserId),
    
}
/// Used when a client is connecting
#[derive(Serializeable, Debug)]
pub enum ServerConnectionMessage{
    AssignUserId(UserId),
    AcknowledgeId,
    IdAlreadyInUse,
}


#[derive(Serializeable, Debug)]
pub enum ServerUdpMessage {
    ChatMessage(String),
}

