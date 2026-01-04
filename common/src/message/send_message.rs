use std::net::SocketAddr;
use serializeable::Serializeable;
use tokio::io::AsyncWriteExt;
use tokio::net::UdpSocket;
use crate::message::{ClientTcpMessage, ClientUdpMessage, ServerTcpMessage, ServerUdpMessage};
use crate::message::client_message::ClientConnectionMessage;
use crate::message::server_message::ServerConnectionMessage;

impl ClientUdpMessage{
    pub async fn send(&self, udp: UdpSocket) -> std::io::Result<usize> {
        let bytes = self.serialize();
        udp.send(&bytes).await
    }
}

impl ServerUdpMessage{
    pub async fn send_to(&self, udp_socket: UdpSocket, target: SocketAddr) -> std::io::Result<usize> {
        let bytes = self.serialize();
        udp_socket.send_to(&bytes, target).await
    }
}

pub trait TcpSendable: Serializeable {
    async  fn send<T: AsyncWriteExt + Unpin>(&self, tcp: &mut T) -> std::io::Result<()> {
        let bytes = self.serialize();
        tcp.write_all(&bytes).await
    }
}

impl TcpSendable for ServerTcpMessage {}
impl TcpSendable for ServerConnectionMessage {}
impl TcpSendable for ClientTcpMessage {}
impl TcpSendable for ClientConnectionMessage {}