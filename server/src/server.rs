use std::collections::HashMap;
use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::net::ToSocketAddrs;
use common::UserId;
use crate::network_interface::NetworkInterface;

pub(crate) struct Server {
    network_interface: NetworkInterface,
    state: ServerState,
    last_tick: Instant,
}

impl Server {
    const TICK_INTERVAL: Duration = Duration::from_millis(10);
    pub(crate) async fn new<A: ToSocketAddrs>(addr: A) -> Self {
        let network_interface = NetworkInterface::create(addr);

        Self{
            state: Default::default(),
            network_interface,
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