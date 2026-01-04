use tokio::sync::mpsc::error::TryRecvError;
use common::message::{ServerMessage, ServerTcpMessage, ServerUdpMessage};
use crate::client::Client;

impl Client {
    pub fn handle_incoming_messages(&mut self) {
        loop{
            match self.incoming_messages.try_recv(){
                Ok(ServerMessage::Tcp(msg)) => self.handle_tcp_message(msg),
                Ok(ServerMessage::Udp(msg)) => self.handle_udp_message(msg),
                Err(TryRecvError::Disconnected) => panic!("Network manager shut down unexpectedly"),
                Err(TryRecvError::Empty) => break,
            }
        }
    }

    fn handle_tcp_message(&mut self, message: ServerTcpMessage) {
        match message {
            _ => unimplemented!(),
        }
    }

    fn handle_udp_message(&mut self, message: ServerUdpMessage) {
        match message {
            _ => unimplemented!(),
        }
    }
}