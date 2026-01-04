use serializeable::Serializeable;
use crate::UserId;

#[derive(Serializeable, Debug)]
pub enum ClientTcpMessage {
    Text(String),
}

#[derive(Serializeable, Debug)]
pub enum ClientConnectionMessage{
    ConnectNew,
    ConnectWithId(UserId),
}


#[derive(Serializeable, Debug)]
pub enum ClientUdpMessage {
    ChatMessage(String),
}

