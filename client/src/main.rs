mod client_network_manager;
mod client;
mod message_resolver;

use common::SERVER_ADDR;
use crate::client::Client;


#[tokio::main]
async fn main() -> std::io::Result<()> {
    let client = Client::new(SERVER_ADDR).await?;

    client.run();
    Ok(())
}