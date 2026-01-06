use common::message::{ServerMessage, ServerTcpMessage, ServerUdpMessage};
use crate::client::Client;

impl Client {
    pub fn handle_incoming_messages(&mut self) {
        while let Some(message) = self.network_interface.incoming_message() {
            match message {
                ServerMessage::Tcp(msg) => self.handle_tcp_message(msg),
                ServerMessage::Udp(msg) => self.handle_udp_message(msg),
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