use serializeable::Serializeable;
use tokio::net::{TcpStream, ToSocketAddrs, UdpSocket};
use common::{ServerTcpMessage, ServerUdpMessage};
use crate::state::State;

pub(crate) struct Client{
    state: State,
    receiver: i32,
}



impl Client {
}

impl State{
    fn new() -> Self {
        Self{

        }
    }
}

