use tokio::net::ToSocketAddrs;
use crate::network_interface::NetworkInterface;

pub(super) struct Client{
    pub network_interface: NetworkInterface,
}

impl Client {
    /// Creates a new Client Instance and connects to the provided Address
    pub async fn new<A: ToSocketAddrs>(server_address: A) -> std::io::Result<Self> {
        let interface = NetworkInterface::create(server_address).await?;
        Ok(Self{
            network_interface: interface
        })
    }

    pub fn run(mut self) {
        loop{
            self.handle_incoming_messages();
            //client loop goes here (such as rendering)
        }
    }
}