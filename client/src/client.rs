use tokio::net::ToSocketAddrs;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use common::message::{ClientMessage, ClientTcpMessage, ClientUdpMessage, ServerMessage};
use crate::client_network_manager::ClientNetworkManager;
use crate::game_state::ClientGameState;
use eframe::{egui, App, NativeOptions};
use egui::{Color32, Key, Stroke};
use nalgebra::Vector2;
use common::game_state::PlayerState;
use common::game_state::PlayerState::Alive;

pub(super) struct Client{
    pub(crate) incoming_messages: UnboundedReceiver<ServerMessage>,
    pub(crate) outgoing_messages: UnboundedSender<ClientMessage>,
    pub(crate) client_state: ClientGameState,
}

impl Client {
    /// Creates a new Client Instance and connects to the provided Address
    pub async fn new<A: ToSocketAddrs>(server_address: A) -> std::io::Result<Self> {
        let (outgoing_messages, incoming_messages) = ClientNetworkManager::new(server_address).await?;
        Ok(Self{
            incoming_messages, outgoing_messages,
            client_state: ClientGameState::new(),
        })
    }

    pub fn run(mut self) {
        eframe::run_native(
            "Bgario",
            NativeOptions::default(),
            Box::new(|_cc| Ok(Box::new(self)))
        ).unwrap();
    }
}

impl App for Client {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {

            self.handle_incoming_messages();

            let painter = ui.painter();

            if let Alive(player) = &mut self.client_state.player {

                let (w, a, s, d) = ctx.input(|i|
                    //i.pointer.hover_pos()
                    (
                        i.key_down(Key::W),
                        i.key_down(Key::A),
                        i.key_down(Key::S),
                        i.key_down(Key::D),
                    )
                );
                const SPEED: f32 = 0.1;
                let dir = match (w, s) {
                    (true, false) => Vector2::new(0., -1.),
                    (false, true) => Vector2::new(0., 1.),
                    _ => Vector2::zeros(),
                } + match (a, d) {
                    (true, false) => Vector2::new(-1., 0.),
                    (false, true) => Vector2::new(1., 0.),
                    _ => Vector2::zeros(),
                };

                let movement = dir.cap_magnitude(1.) * SPEED;
                player.position += movement;
                if movement.magnitude_squared() > 0. {
                    self.outgoing_messages.send(ClientMessage::Udp(ClientUdpMessage::Move {new_pos: player.position})).unwrap();
                }
                painter.circle_filled(player.position.data.0[0].into(), player.size.into(), Color32::LIGHT_GREEN);
            }
            else {
                if ui.button("Respawn").clicked() {
                    self.outgoing_messages.send(ClientMessage::Tcp(ClientTcpMessage::Respawn)).expect("Network manager for Tcp hung up"); //ask to be respawned if dead
                }
            }
            let painter = ui.painter();

            for player_state in &self.client_state.other_players {
                if let PlayerState::Alive(player) = player_state {
                    painter.circle_filled(
                        player.position.data.0[0].into(),
                        player.size.into(),
                        Color32::DARK_GRAY,
                    );
                }
            }

            for food in &self.client_state.foods {
                painter.circle(
                    food.position.data.0[0].into(),
                    0.5,
                    Color32::LIGHT_GREEN,
                    Stroke::new(1.0, Color32::WHITE),
                );
            }
        });
    }
}