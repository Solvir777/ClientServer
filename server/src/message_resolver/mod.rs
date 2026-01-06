use tokio::sync::mpsc::error::TryRecvError;
use common::message::{ClientMessage, ClientTcpMessage, ClientUdpMessage};
use common::UserId;
use crate::server::Server;

impl Server {
    pub fn handle_incoming_messages(&mut self) {
        loop{
            match self.incoming_messages.try_recv() {
                Ok((ClientMessage::Tcp(msg), id)) => self.handle_tcp_message(msg, id),
                Ok((ClientMessage::Udp(msg), id)) => self.handle_udp_message(msg, id),
                Err(TryRecvError::Disconnected) => {panic!("Network manager shut down unexpectedly")}
                Err(TryRecvError::Empty) => break,
            }
        }
        while let Ok((message, userid)) = self.incoming_messages.try_recv() {
            match message {
                ClientMessage::Tcp(message) => {
                    self.handle_tcp_message(message, userid);
                }
                ClientMessage::Udp(message) => {
                    self.handle_udp_message(message, userid);
                }
            }
        }
    }
    
    fn handle_tcp_message(&mut self, message: ClientTcpMessage, userid: UserId) {
        match message {
            ClientTcpMessage::Respawn => {
                println!("player is getting respawned!");
                self.state.spawn_player(userid);
            }
            _ => unimplemented!(),
        }
    }
    
    fn handle_udp_message(&mut self, message: ClientUdpMessage, userid: UserId) {
        match message {
            ClientUdpMessage::Move{new_pos} => {self.state.move_player(userid, new_pos);}
            _ => unimplemented!(),
        }
    }
}