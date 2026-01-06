use common::SERVER_ADDR;
use crate::server::Server;

mod server;
mod message_resolver;
mod network_interface;

#[tokio::main]
async fn main() {
    let server = Server::new(SERVER_ADDR).await;

    server.run();
}


