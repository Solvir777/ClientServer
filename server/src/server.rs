use std::collections::HashMap;
use std::net::SocketAddr;
use tokio::sync::mpsc::{unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use tokio::net::{TcpListener, TcpStream, ToSocketAddrs};
use common::{ClientMessage, ClientTcpMessage, ServerMessage, ServerTcpMessage};
use common::ServerTcpMessage::Text;
use crate::server_network_manager::ServerNetworkManager;

pub type UserId = u64;

pub(crate) struct Server {
    state: ServerState,
    incoming_messages: Receiver<(ClientMessage, UserId)>,
    outgoing_messages: Sender<(ServerMessage, UserId)>
}

impl Server {
    pub(crate) fn new<A: ToSocketAddrs>(addr: A) -> Self {
        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();


        ServerNetworkManager::run(addr, out_rx, in_tx);

        Self{
            state: Default::default(),
            incoming_messages: in_rx,
            outgoing_messages: out_tx,
        }
    }

    pub(crate) fn run(mut self) {
        loop {
            if let Some((msg, sender)) = self.incoming_messages.blocking_recv() {
                println!("Received {:?} from {:?}", msg, sender);
                match msg {
                    ClientMessage::ClientTcpMessage(ClientTcpMessage::Text(content)) => {
                        let a = content.clone() + ". Returning the Favor!";
                        self.outgoing_messages.send((ServerMessage::ServerTcpMessage(ServerTcpMessage::Text(a)), sender)).unwrap();
                    }
                    ClientMessage::ClientUdpMessage(msg) => {
                        println!("received udp: {:?}", msg);
                    }
                    _ => {
                        println!("unrecognized client message: {:?}", msg);
                    }
                }
            }
        }
    }

    fn message_response(&mut self, message: (ClientMessage, UserId)) {
        let (msg, user_id) = message;
        match msg {
            ClientMessage::ClientTcpMessage(tcp) => {}
            ClientMessage::ClientUdpMessage(udp) => {}
            ClientMessage::ClientLost => {}
        }
    }
}

struct Client {
    name: String,
    id: UserId,
}

#[derive(Default)]
struct ServerState {
    users: HashMap<UserId, Client>,
}