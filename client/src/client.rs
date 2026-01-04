use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use common::message::{ClientMessage, ServerMessage};
use crate::client_network_manager::ClientNetworkManager;

pub(super) struct Client{
    pub(crate) incoming_messages: UnboundedReceiver<ServerMessage>,
    outgoing_messages: UnboundedSender<ClientMessage>,
}

impl Client {
    /// Creates a new Client Instance and connects to the provided Address
    pub async fn new<A: ToSocketAddrs>(server_address: A) -> std::io::Result<Self> {
        let (outgoing_messages, incoming_messages) = ClientNetworkManager::new(server_address).await?;
        Ok(Self{
            incoming_messages, outgoing_messages,
        })
    }

    pub fn run(mut self) {
        loop{
            self.handle_incoming_messages();
            //client loop goes here (such as rendering)
        }
    }
}