mod client_handler;
use std::collections::{HashMap, HashSet};
use std::net::SocketAddr;
use std::sync::{Arc};
use tokio::sync::{Mutex, RwLock};
use serializeable::Serializeable;
use tokio::net::{TcpListener, ToSocketAddrs, UdpSocket};
use common::message::{ClientMessage, ClientUdpMessage, ServerMessage};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver as Receiver, UnboundedSender as Sender, UnboundedSender};
use common::UserId;
use crate::network_interface::ClientEvent;
use crate::network_interface::network_manager::client_handler::ClientHandler;

pub(super) struct NetworkManager {
    socket_addr_to_user_id: Arc<RwLock<HashMap<SocketAddr, UserId>>>,
    user_id_to_message_sender: Arc<RwLock<HashMap<UserId, UnboundedSender<ServerMessage>>>>,

    tcp_listener: TcpListener,
    udp_socket: Arc<UdpSocket>,

    outgoing_messages: Receiver<(ServerMessage, UserId)>,
    incoming_messages: Sender<(ClientEvent, UserId)>
}


impl NetworkManager {

    pub(super) async fn launch<A: ToSocketAddrs>(
        addr: A,
    ) -> (Sender<(ServerMessage, UserId)>, Receiver<(ClientEvent, UserId)>){
        let tcp_listener = TcpListener::bind(&addr).await.unwrap();
        let udp = UdpSocket::bind(addr).await.unwrap();
        let (in_tx, in_rx) = unbounded_channel();
        let (out_tx, out_rx) = unbounded_channel();
        
        Self{
            socket_addr_to_user_id: Default::default(),
            user_id_to_message_sender: Default::default(),
            tcp_listener,
            udp_socket: Arc::new(udp),
            incoming_messages: in_tx,
            outgoing_messages: out_rx,
        }.run();
        
        (out_tx, in_rx)
    }

    ///Call this to start accepting clients
    pub(super) fn run(self){
        tokio::spawn(Self::accept_clients(self.tcp_listener, self.incoming_messages.clone(), self.udp_socket.clone(), self.user_id_to_message_sender.clone(), self.socket_addr_to_user_id.clone()));
        tokio::spawn(Self::receive_messages_udp(self.udp_socket.clone(), self.incoming_messages.clone(), self.socket_addr_to_user_id.clone()));
        tokio::spawn(Self::distribute_messages(self.user_id_to_message_sender, self.outgoing_messages));
    }
    
    
    /// Distributes messages to their respective client thread to be send. \
    /// This will not return
    async fn distribute_messages(user_id_to_message_sender: Arc<RwLock<HashMap<UserId, UnboundedSender<ServerMessage>>>>, mut outgoing_messages: Receiver<(ServerMessage, UserId)>) {
        while let Some((message, user_id)) = outgoing_messages.recv().await {
            if let Some(sender) = user_id_to_message_sender.read().await.get(&user_id) {
                sender.send(message).unwrap();
            }
        }
    }
    
    /// Open a TcpListener and spawn a client handler for every incoming connection. \
    /// This will not return
    async fn accept_clients(
        listener: TcpListener,
        incoming_messages: Sender<(ClientEvent, UserId)>,
        udp: Arc<UdpSocket>,
        message_senders: Arc<RwLock<HashMap<UserId, UnboundedSender<ServerMessage>>>>,
        addr_to_user_id: Arc<RwLock<HashMap<SocketAddr, UserId>>>,
    ) {
        let connected_ids: Arc<Mutex<HashSet<UserId>>> = Default::default();
        loop { 
            let client_stream= listener.accept().await.unwrap().0;
            
            ClientHandler::spawn(udp.clone(), client_stream, incoming_messages.clone(), message_senders.clone(), addr_to_user_id.clone(), connected_ids.clone());
        } 
    }

    /// Spawn once to receive messages over udp. \
    /// This will not return
    async fn receive_messages_udp(udp: Arc<UdpSocket>, incoming_messages: Sender<(ClientEvent, UserId)>, addr_to_user_id: Arc<RwLock<HashMap<SocketAddr, UserId>>>) {
        let mut buf = [0u8; 2048];
        loop {
            let (n, sender) = udp.recv_from(&mut buf).await.unwrap();
            let msg = ClientUdpMessage::deserialize(&mut &buf[..n]).unwrap();
            println!("socket_addr: {sender}, list: {:?}", addr_to_user_id.read().await);

            if let Some(id) = addr_to_user_id.read().await.get(&sender) {
                incoming_messages.send((ClientEvent::ClientMessage(ClientMessage::Udp(msg)), *id)).unwrap();
            } else{ println!("Received message from unknown client. msg: {:?}", msg); }
        }
    }
}