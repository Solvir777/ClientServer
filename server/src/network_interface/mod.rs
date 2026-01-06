mod network_manager;

use std::net::ToSocketAddrs;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use common::message::{ClientMessage, ServerMessage, ServerTcpMessage, ServerUdpMessage};
use common::UserId;
use crate::network_interface::network_manager::NetworkManager;

pub enum ClientEvent{
    Connected,
    Disconnected,
    ClientMessage(ClientMessage),
}


pub(super) struct NetworkInterface{
    incoming_messages: UnboundedReceiver<(ClientEvent, UserId)>,
    outgoing_messages: UnboundedSender<(ServerMessage, UserId)>,
}

impl NetworkInterface{
    const ERROR_MSG: &str = "Servers Network Manager crashed unexpectedly";

    /// Create a new ServerNetworkManager and return an Interface for it.
    pub async fn create<A: ToSocketAddrs>(addr: A) -> Self {
        let (out_tx, in_rx) = NetworkManager::launch(addr).await;

        Self{
            outgoing_messages: out_tx,
            incoming_messages: in_rx,
        }
    }

    pub fn send_tcp(&mut self, msg: ServerTcpMessage, target: UserId){
        self.outgoing_messages.send((ServerMessage::Tcp(msg), target)).expect(Self::ERROR_MSG)
    }
    pub fn send_udp(&mut self, msg: ServerUdpMessage, target: UserId){
        self.outgoing_messages.send((ServerMessage::Udp(msg), target)).expect(Self::ERROR_MSG)
    }

    /// A return value of None means that no more Messages have been received _yet_.
    pub fn incoming_message(&mut self) -> Option<(ClientEvent, UserId)> {
        match self.incoming_messages.try_recv() {
            Ok(content) => Some(content),
            Err(tokio::sync::mpsc::error::TryRecvError::Disconnected) => panic!(Self::ERROR_MSG),
            Err(tokio::sync::mpsc::error::TryRecvError::Empty) => None
        }
    }

}