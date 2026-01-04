use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use tokio::net::ToSocketAddrs;
use common::message::{ClientMessage, ServerMessage};
use common::UserId;
use crate::server_network_manager::ServerNetworkManager;


pub(crate) struct Server {
    state: ServerState,
    pub(super) incoming_messages: Receiver<(ClientMessage, UserId)>,
    outgoing_messages: Sender<(ServerMessage, UserId)>,
    last_tick: Instant,
}

impl Server {
    const TICK_INTERVAL: Duration = Duration::from_millis(10);
    pub(crate) async fn new<A: ToSocketAddrs>(addr: A) -> Self {
        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();

        ServerNetworkManager::new(addr, out_rx, in_tx).await
            .run();

        Self{
            state: Default::default(),
            incoming_messages: in_rx,
            outgoing_messages: out_tx,
            last_tick: Instant::now(),
        }
    }

    pub(crate) fn run(mut self) {
        loop {
            self.suspend_until_next_tick();
            self.handle_incoming_messages();
        }
    }

    fn suspend_until_next_tick(&mut self) {
        let diff = self.last_tick.elapsed();
        let time_until_tick = Self::TICK_INTERVAL - diff;
        if time_until_tick.as_millis() > 0 {
            sleep(time_until_tick);
        }
        self.last_tick = Instant::now();
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