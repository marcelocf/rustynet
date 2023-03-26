use echonet_lite::{prelude::ClassPacket, ElPacket};
use std::{
    io,
    net::{IpAddr, Ipv4Addr},
};
use tokio::net::UdpSocket;
use tracing::info;

pub mod data;
pub mod discovery;

pub const EL_MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 23, 0);

// TODO: this probably need the following refactoring:
// mod echonet::io{Request, Response,server::{Server}}
// and then all the complex types within their own modules

// Represents an Echonet Lite request. Including the packet and source address
pub struct Request {
    packet: ElPacket,
    source: IpAddr,
}

impl Request {
    fn new(packet: ElPacket, source: IpAddr) -> Self {
        Request { packet, source }
    }
}

impl From<Request> for ElPacket {
    fn from(req: Request) -> ElPacket {
        req.packet
    }
}

impl From<Request> for IpAddr {
    fn from(req: Request) -> Self {
        req.source
    }
}

// Response to echonet lite requests.
pub struct Response {
    packet: ElPacket,
    destination: IpAddr,
}

impl Response {
    fn new(packet: ElPacket, destination: IpAddr) -> Self {
        Self {
            packet,
            destination,
        }
    }
}

// Represents an Echonet Lite server

pub struct Server {
    socket: UdpSocket,
}

trait Listener {
    // a listener can have multiple responses
    // for example, when sending a discovery package it can trigger multiple
    // different messages
    fn process(request: Request) -> Vec<Response>;
}

// Server implementation using Tokio for async code
impl Server {
    async fn new() -> io::Result<Self> {
        let socket = UdpSocket::bind("0.0.0.0:3610").await?;
        socket.set_multicast_loop_v4(true)?;
        socket.join_multicast_v4(EL_MULTICAST_ADDR, [0, 0, 0, 0].into())?;

        Ok(Server { socket })
    }

    async fn listen(self) -> io::Result<()> {
        loop {
            let mut buffer = [0u8; 1024];
            match self.socket.recv_from(&mut buffer).await {
                Err(_) => break,
                Ok((_, src_addr)) => {
                    if let Ok((_, packet)) = ElPacket::from_bytes(&buffer) {
                        let obj: ClassPacket = packet.into();
                        info!("got response from {}", src_addr);
                        info!("{:}", obj);
                    }
                }
            }
        }

        Ok(())
    }
}
