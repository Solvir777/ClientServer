use tokio::io::AsyncWriteExt;
pub use crate::message::client_message::{ClientTcpMessage, ClientUdpMessage};
pub use crate::message::server_message::{ServerTcpMessage, ServerUdpMessage};

pub mod server_message;
pub mod client_message;
pub mod send_message;

#[derive(Debug)]
pub enum ServerMessage{
    Tcp(ServerTcpMessage),
    Udp(ServerUdpMessage),
}

#[derive(Debug)]
pub enum ClientMessage{
    Tcp(ClientTcpMessage),
    Udp(ClientUdpMessage),
}