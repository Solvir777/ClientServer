use common::message::send_message::TcpSendable;
use std::collections::{HashMap, HashSet};
use std::hash::{DefaultHasher, Hash, Hasher};
use std::net::SocketAddr;
use common::UserId;
use common::message::{ClientMessage, ClientTcpMessage, ServerMessage, ServerTcpMessage, ServerUdpMessage};
use serializeable::Serializeable;
use std::sync::Arc;
use tokio::io::{AsyncWriteExt};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::net::{TcpStream, UdpSocket};
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::mpsc::{UnboundedReceiver as Receiver, UnboundedSender as Sender};
use tokio::sync::{Mutex, RwLock};
use common::message::client_message::ClientConnectionMessage;
use common::message::server_message::ServerConnectionMessage;

pub struct ClientHandler {
    id: UserId,
    udp: Arc<UdpSocket>,
    tcp_writer: OwnedWriteHalf,
    tcp_reader: OwnedReadHalf,
    incoming_messages: Sender<(ClientMessage, UserId)>, //Only for TCP.
    outgoing_messages: Receiver<ServerMessage>,
}


impl ClientHandler {
    pub async fn login_procedure(tcp: &mut TcpStream, connected_ids: Arc<Mutex<HashSet<UserId>>>) -> UserId {
        async fn create_user_id(addr: &SocketAddr, connected_ids: Arc<Mutex<HashSet<UserId>>>) -> UserId {
            let mut hasher = DefaultHasher::new();
            addr.hash(&mut hasher);
            let mut id = hasher.finish() as UserId;
            while connected_ids.lock().await.contains(&id) {
                id += 1;
            }
            id
        }


        loop {
            match ClientConnectionMessage::async_deserialize(tcp).await.unwrap() {
                ClientConnectionMessage::ConnectNew => {
                    let id = create_user_id(&tcp.peer_addr().unwrap(), connected_ids.clone()).await;
                    connected_ids.lock().await.insert(id);
                    ServerConnectionMessage::AssignUserId(id).send(tcp).await.unwrap();
                    return id;
                },
                ClientConnectionMessage::ConnectWithId(requested_id) => {
                    if connected_ids.lock().await.insert(requested_id) {
                        ServerConnectionMessage::AcknowledgeId.send(tcp).await.unwrap();
                        return requested_id;
                    } else {
                        ServerConnectionMessage::IdAlreadyInUse.send(tcp).await.unwrap();
                        //let the client start a new connection attempt
                    }
                },
            }
        }
    }
    pub fn spawn(
        udp: Arc<UdpSocket>,
        mut tcp: TcpStream,
        incoming_messages: UnboundedSender<(ClientMessage, UserId)>,
        outgoing_message_writers: Arc<RwLock<HashMap<UserId, UnboundedSender<ServerMessage>>>>,
        addr_to_user_id: Arc<RwLock<HashMap<SocketAddr, UserId>>>,
        connected_users: Arc<Mutex<HashSet<UserId>>>,
    ) {
        tokio::spawn(
            async move {
                {
                    let id = ClientHandler::login_procedure(&mut tcp, connected_users.clone()).await;

                    let (tcp_reader, tcp_writer) = tcp.into_split();
                    let (outgoing_per_client_tx, outgoing_per_client_rx) = unbounded_channel::<ServerMessage>();


                    addr_to_user_id.write().await.insert(tcp_reader.peer_addr().unwrap(), id);
                    outgoing_message_writers.write().await.insert(id, outgoing_per_client_tx);

                    Self {
                        id,
                        udp,
                        tcp_writer,
                        tcp_reader,
                        incoming_messages,
                        outgoing_messages: outgoing_per_client_rx,
                    }
                }
                    .run() 
            }
        );
    }

    async fn run(mut self) {
        let (tcp_message_sender, tcp_message_receiver) = unbounded_channel::<ServerTcpMessage>();
        let (udp_message_sender, udp_message_receiver) = unbounded_channel::<ServerUdpMessage>();
        
        tokio::spawn(Self::receive_tcp(self.tcp_reader, self.incoming_messages, self.id));
        tokio::spawn(Self::send_udp(udp_message_receiver, self.udp, self.tcp_writer.peer_addr().unwrap()));
        tokio::spawn(Self::send_tcp(tcp_message_receiver, self.tcp_writer));
        while let Some(msg) = self.outgoing_messages.recv().await {
            match msg {
                ServerMessage::Tcp(tcp_msg) => {tcp_message_sender.send(tcp_msg).unwrap();}
                ServerMessage::Udp(udp_msg) => {udp_message_sender.send(udp_msg).unwrap();}
            }
        }
    }
    
    async fn receive_tcp(mut tcp_reader: OwnedReadHalf, incoming_messages: Sender<(ClientMessage, UserId)>, id: UserId) {
        while let Ok(msg) = ClientTcpMessage::async_deserialize(&mut tcp_reader).await {
            incoming_messages.send((ClientMessage::Tcp(msg), id)).unwrap();
        }
    }
    
    async fn send_tcp(mut receiver: Receiver<ServerTcpMessage>, mut tcp_writer: OwnedWriteHalf) {
        while let Some(tcp_message) = receiver.recv().await {
            let bytes = tcp_message.serialize();
            tcp_writer.write_all(&bytes).await.unwrap();
        }
    }
    
    async fn send_udp(mut receiver: Receiver<ServerUdpMessage>, udp: Arc<UdpSocket>, socket_addr: SocketAddr) {
        while let Some(udp_message) = receiver.recv().await {
            let bytes = udp_message.serialize();
            udp.send_to(&bytes, socket_addr).await.unwrap();
        }
    }
}
