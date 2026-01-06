use common::SERVER_ADDR;
use crate::server::Server;

mod server;
mod server_network_manager;
mod message_resolver;
mod game_state;

#[tokio::main]
async fn main() {
    let server = Server::new(SERVER_ADDR).await;

    server.run::<50>();
}


