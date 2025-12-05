use std::io::Error;
use std::sync::mpsc;
use serializeable::Serializeable;
use tokio::net::{TcpStream, ToSocketAddrs, UdpSocket};
use common::{ClientMessage, ClientUdpMessage, ServerMessage, ServerTcpMessage, ServerUdpMessage};
/*
pub(crate) struct MessageReceiver {
    udp: UdpSocket,
    tcp: TcpStream,
    msg_sender: mpsc::Sender<ServerMessage>,
}


impl MessageReceiver {
    pub async fn new<A: ToSocketAddrs>(addr: A) -> (Self, mpsc::Receiver<ServerMessage>) {
        let udp = UdpSocket::bind(&addr).await.unwrap();
        let tcp = TcpStream::connect(addr).await.unwrap();

        let (msg_sender, msg_receiver) = mpsc::channel();

        (
            Self {
                udp,
                tcp,
                msg_sender,
            },
            msg_receiver,
        )
    }
    pub async fn run(self) {
        tokio::spawn(receive_udp(self.msg_sender.clone(), self.udp));
        tokio::spawn(receive_tcp(self.msg_sender.clone(), self.tcp));
    }
}


async fn receive_udp(channel_sender: mpsc::Sender<ServerMessage>, udp: UdpSocket) {
    let mut buf = [0u8; 1024];
    loop{
        if let Ok(n) = udp.recv(&mut buf).await {
            let bytes = buf[..n].to_vec();
            match ServerUdpMessage::async_deserialize(&mut bytes.as_slice()).await {
                Ok(msg) => {
                    channel_sender.send(ServerMessage::ServerUdpMessage(msg)).unwrap();
                }
                Err(_) => {continue;}
            }

        }
        else {break}
    }
}

async fn receive_tcp(channel_sender: mpsc::Sender<ServerMessage>, mut tcp: TcpStream) {
    loop {
        let des = ServerTcpMessage::async_deserialize(&mut tcp).await;
        match des {
            Ok(msg) if channel_sender.send(ServerMessage::ServerTcpMessage(msg)).is_ok() => {
                channel_sender.send(msg).unwrap();
            }
            
        }
        if let Err(_) = channel_sender.send(ServerMessage::ServerTcpMessage(msg)) {
            break;
        }
    }
}*/