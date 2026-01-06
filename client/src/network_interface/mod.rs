mod network_manager;

use std::net::ToSocketAddrs;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use tokio::sync::mpsc::error::TryRecvError;
use common::message::{ClientMessage, ClientTcpMessage, ClientUdpMessage, ServerMessage};
use crate::network_interface::network_manager::NetworkManager;

pub(super) struct NetworkInterface {
    incoming_messages: UnboundedReceiver<ServerMessage>,
    outgoing_messages: UnboundedSender<ClientMessage>,
}

impl NetworkInterface {
    const ERROR_MSG: &str = "Clients Network Manager crashed unexpectedly";
    pub async fn create<A: ToSocketAddrs>(addr: A) -> std::io::Result<Self> {
        let (outgoing_messages, incoming_messages) = NetworkManager::launch(addr).await?;
        Ok(Self { incoming_messages, outgoing_messages })
    }

    pub fn send_tcp(&mut self, msg: ClientTcpMessage){
        self.outgoing_messages.send(ClientMessage::Tcp(msg)).expect(Self::ERROR_MSG)
    }
    pub fn send_udp(&mut self, msg: ClientUdpMessage){
        self.outgoing_messages.send(ClientMessage::Udp(msg)).expect(Self::ERROR_MSG)
    }

    /// A return value of None means that no more Messages have been received _yet_.
    pub fn incoming_message(&mut self) -> Option<ServerMessage> {
        match self.incoming_messages.try_recv() {
            Ok(content) => Some(content),
            Err(TryRecvError::Disconnected) => panic!("{}", Self::ERROR_MSG),
            Err(TryRecvError::Empty) => None
        }
    }
}

