use common::SERVER_ADDR;
use crate::server::Server;

mod server;
mod server_network_manager;
mod message_resolver;

#[tokio::main]
async fn main() {
    let server = Server::new(SERVER_ADDR).await;

    tokio::task::spawn_blocking(
        || server.run()
    );
}


