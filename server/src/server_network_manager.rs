mod client_thread;

use bimap::BiMap;
use std::collections::HashMap;
use std::hash::{DefaultHasher, Hash};
use std::net::SocketAddr;
use std::sync::{Arc};
use tokio::sync::Mutex;
use serializeable::Serializeable;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpListener, ToSocketAddrs, UdpSocket};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use common::{ClientMessage, ClientTcpMessage, ClientUdpMessage, ServerMessage};
use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use crate::server::UserId;

pub(crate) struct ServerNetworkManager{
    addr_to_user_id: Arc<Mutex<BiMap<SocketAddr, UserId>>>,
    user_id_to_tcp_writer: Arc<Mutex<HashMap<UserId, OwnedWriteHalf>>>,

    listener: TcpListener,
    udp: Arc<UdpSocket>,

    incoming_messages: Sender<(ClientMessage, UserId)>,
    outgoing_messages: Receiver<(ServerMessage, UserId)>
}


impl ServerNetworkManager {
    async fn new<A: ToSocketAddrs>(
        addr: A,
        messages_outgoing: Receiver<(ServerMessage, UserId)>,
        messages_incoming: Sender<(ClientMessage, UserId)>,
    ) -> Self{
        let listener = TcpListener::bind(&addr).await.unwrap();
        let udp = UdpSocket::bind(addr).await.unwrap();
        println!("Server listening on: {}", udp.local_addr().unwrap());

        Self{
            addr_to_user_id: Default::default(),
            user_id_to_tcp_writer: Default::default(),
            listener,
            udp: Arc::new(udp),
            incoming_messages: messages_incoming,
            outgoing_messages: messages_outgoing,
        }
    }

    ///Calling this creates a new Thread that will receive and send messages to Clients, based on their UserId.

    pub fn run<A: ToSocketAddrs>(
        addr: A,
        outgoing_messages: Receiver<(ServerMessage, UserId)>,
        incoming_messages: Sender<(ClientMessage, UserId)>
    ){
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build().unwrap();

        let this = rt.block_on(
            Self::new(&addr, outgoing_messages, incoming_messages)
        );

        let a = rt.spawn(Self::accept_clients(this.listener, this.incoming_messages.clone(), this.addr_to_user_id.clone(), this.user_id_to_tcp_writer.clone()));
        let b = rt.spawn(Self::receive_messages_udp(this.udp.clone(), this.incoming_messages, this.addr_to_user_id.clone()));
        let c = rt.spawn(Self::send_messages(this.outgoing_messages, this.addr_to_user_id.clone(), this.user_id_to_tcp_writer.clone(), this.udp.clone()));
        let _ = std::thread::spawn(
            move || {
                rt.block_on(
                    async {
                        tokio::join!(a, b, c);
                        println!("Server exiting");
                    }
                );
            }
        );
    }


    /// Sends out messages received over the messages_outgoing Channel
    async fn send_messages(
        mut messages_outgoing: Receiver<(ServerMessage, UserId)>,
        addr_to_user_id: Arc<Mutex<BiMap<SocketAddr, UserId>>>,
        user_id_to_tcp_writer: Arc<Mutex<HashMap<UserId, OwnedWriteHalf>>>,
        udp: Arc<UdpSocket>,
    ) { loop {
        if let Some((message, user_id)) = messages_outgoing.recv().await {
            match message {
                ServerMessage::ServerTcpMessage(msg) => {
                    if let Some(writer) = user_id_to_tcp_writer.lock().await.get_mut(&user_id) {
                        let bytes = msg.serialize();
                        writer.write_all(&bytes).await.unwrap();
                    }
                    else{
                        println!("UserId {user_id} not recognized. Following message was not sent: {:?}", msg);
                    }
                }
                ServerMessage::ServerUdpMessage(msg) => {
                    if let Some(addr) = addr_to_user_id.lock().await.get_by_right(&user_id) {
                        udp.send_to(&msg.serialize(), addr).await.unwrap();
                    }
                    else {
                        println!("couldnt send to user via udp");
                    }
                }
            }
        } else { break; }
    } }


    ///Open a TcpListener and spawn a message receiver for every incoming connection
    async fn accept_clients(
        listener: TcpListener,
        incoming_messages: Sender<(ClientMessage, UserId)>,
        addr_to_user_id: Arc<Mutex<BiMap<SocketAddr, UserId>>>,
        user_id_to_tcp_writer: Arc<Mutex<HashMap<UserId, OwnedWriteHalf>>>,
    ) { loop {
        let (client, addr) = listener.accept().await.unwrap();
        let id = hash_addr(&addr);
        addr_to_user_id.lock().await.insert(addr, id);
        let (reader, writer) = client.into_split();
        user_id_to_tcp_writer.lock().await.insert(id, writer);

        println!("using id {} for new client", id);

        tokio::spawn(Self::receive_messages_tcp(reader, incoming_messages.clone(), id));
    } }

    ///Spawn once to receive messages over udp
    async fn receive_messages_udp(udp: Arc<UdpSocket>, incoming_messages: Sender<(ClientMessage, UserId)>, addr_to_user_id: Arc<Mutex<BiMap<SocketAddr, UserId>>>) {
        let mut buf = [0u8; 2048];
        loop {
            let (n, sender) = udp.recv_from(&mut buf).await.unwrap();
            let msg = ClientUdpMessage::deserialize(&mut &buf[..n]).unwrap();
            println!("socket_addr: {sender}, list: {:?}", addr_to_user_id.lock().await);

            if let Some(id) = addr_to_user_id.lock().await.get_by_left(&sender) {
                incoming_messages.send((ClientMessage::ClientUdpMessage(msg), *id)).unwrap();
            } else{ println!("Received message from unknown client. msg: {:?}", msg); }
        }
    }
    ///Spawn per Client
    async fn receive_messages_tcp(mut tcp: OwnedReadHalf, incoming_messages: Sender<(ClientMessage, UserId)>, id: u64) {
        loop{
            let res = ClientTcpMessage::async_deserialize(&mut tcp).await;
            match res {
                Ok(msg) => {
                    incoming_messages.send((ClientMessage::ClientTcpMessage(msg), id)).unwrap();
                }
                Err(err) => {
                    println!("client disconnected with error {err}!");
                    //client disconnected:
                    // todo: remove client from maps
                    break;
                }
            }
        }
    }
}


fn hash_addr(addr: &SocketAddr) -> u64 {
    use std::hash::DefaultHasher;
    use std::hash::{Hasher, Hash};
    let mut hasher = DefaultHasher::new();
    addr.hash(&mut hasher);
    hasher.finish()
}