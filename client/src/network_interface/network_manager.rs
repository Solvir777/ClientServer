use std::sync::Arc;
use serializeable::Serializeable;
use tokio::io::AsyncWriteExt;
use tokio::net::{TcpStream, ToSocketAddrs, UdpSocket};
use tokio::net::tcp::{OwnedReadHalf, OwnedWriteHalf};
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver as Receiver, UnboundedReceiver, UnboundedSender as Sender, UnboundedSender};
use common::message::{ClientMessage, ServerMessage, ServerTcpMessage, ServerUdpMessage};
pub struct NetworkManager {
    tcp: TcpStream,
    udp: Arc<UdpSocket>,

    incoming_messages: Sender<ServerMessage>,
    outgoing_messages: Receiver<ClientMessage>,
}

impl NetworkManager {
    pub async fn launch<A: ToSocketAddrs>(server_addr: A) -> std::io::Result<(UnboundedSender<ClientMessage>, UnboundedReceiver<ServerMessage>)> {
        let (outgoing_messages_sender, outgoing_messages_receiver) = unbounded_channel();
        let (incoming_messages_sender, incoming_messages_receiver) = unbounded_channel();


        let tcp = TcpStream::connect(&server_addr).await?;
        let udp = Arc::new(UdpSocket::bind(tcp.local_addr()?).await?);
        udp.connect(&server_addr).await?;

        Self{ tcp, udp, incoming_messages: incoming_messages_sender, outgoing_messages: outgoing_messages_receiver }.run();
        Ok(
            (
                outgoing_messages_sender,
                incoming_messages_receiver
            )
        )
    }
    fn run(self) {
        let (tcp_reader, tcp_writer) = self.tcp.into_split();

        tokio::spawn(Self::receive_udp(self.udp.clone(), self.incoming_messages.clone()));
        tokio::spawn(Self::receive_tcp(tcp_reader, self.incoming_messages.clone()));
        tokio::spawn(Self::send_messages(tcp_writer, self.udp.clone(), self.outgoing_messages));
    }

    async fn receive_udp(udp: Arc<UdpSocket>, incoming_messages: Sender<ServerMessage>) {
        let mut buf = [0u8; 2048];
        loop {
            udp.recv(&mut buf).await.expect("failed to receive UDP packet");
            let msg = ServerUdpMessage::async_deserialize(&mut &buf[..]).await.unwrap();
            incoming_messages.send(ServerMessage::Udp(msg)).expect("message receiver hung up");
        }
    }
    async fn receive_tcp(mut tcp_reader: OwnedReadHalf, incoming_messages: Sender<ServerMessage>) {
        loop {
            let msg = ServerTcpMessage::async_deserialize(&mut tcp_reader).await.unwrap();
            incoming_messages.send(ServerMessage::Tcp(msg)).expect("message receiver hung up");
        }
    }

    async fn send_messages(mut tcp_writer: OwnedWriteHalf, udp_socket: Arc<UdpSocket>, mut outgoing_messages: Receiver<ClientMessage>) {
        loop{
            let msg = outgoing_messages.recv().await.unwrap();
            match msg {
                ClientMessage::Tcp(tcp_message) => {
                    tcp_writer.writable().await.unwrap();
                    let msg_bytes = tcp_message.serialize();
                    tcp_writer.write_all(&msg_bytes).await.unwrap();
                }
                ClientMessage::Udp(udp_message) => {
                    let msg_bytes = udp_message.serialize();
                    udp_socket.send(&msg_bytes).await.unwrap();
                }
            }
        }
    }
}