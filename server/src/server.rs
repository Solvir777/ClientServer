use std::thread::sleep;
use std::time::{Duration, Instant};
use tokio::sync::mpsc::{unbounded_channel as channel, UnboundedReceiver as Receiver, UnboundedSender as Sender};
use tokio::net::ToSocketAddrs;
use common::game_state::GameStateUpdate;
use common::message::{ClientMessage, ServerMessage, ServerUdpMessage};
use common::UserId;
use crate::game_state::ServerGameState;
use crate::server_network_manager::ServerNetworkManager;


pub(crate) struct Server{
    pub(super) state: ServerGameState,
    pub(super) incoming_messages: Receiver<(ClientMessage, UserId)>,
    outgoing_messages: Sender<(ServerMessage, UserId)>,
    last_tick: Instant,
}

impl Server {
    pub(crate) async fn new<A: ToSocketAddrs>(addr: A) -> Self {
        let (in_tx, in_rx) = channel();
        let (out_tx, out_rx) = channel();

        ServerNetworkManager::new(addr, out_rx, in_tx).await
            .run();

        Self{
            state: ServerGameState::new(),
            incoming_messages: in_rx,
            outgoing_messages: out_tx,
            last_tick: Instant::now(),
        }
    }

    pub(crate) fn run<const TICK_MILLIS: u64>(mut self) {
        loop {
            self.suspend_until_next_tick::<TICK_MILLIS>();
            self.handle_incoming_messages();
            self.state.tick();
            self.send_state_to_players();
        }
    }

    fn send_state_to_players(&mut self) {
        for (id, index) in self.state.user_id_to_index.iter() {
            let state_update = GameStateUpdate{
                you: *index as u8,
                players: self.state.players.clone(),
                foods: self.state.foods.clone(),
            };
            self.outgoing_messages.send(
                (
                    ServerMessage::Udp(ServerUdpMessage::GameState(state_update)),
                    *id
                )
            ).unwrap()
        }
    }
    fn suspend_until_next_tick<const TICK_MILLIS: u64>(&mut self) {
        let tick_length: Duration = Duration::from_millis(TICK_MILLIS);
        let since_last_tick = self.last_tick.elapsed();
        let time_until_tick = tick_length.saturating_sub(since_last_tick);
        if time_until_tick.as_millis() > 0 {
            sleep(time_until_tick);
        }
        self.last_tick = Instant::now();
    }
}