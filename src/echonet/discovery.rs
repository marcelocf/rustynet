// Handles the discovery of new devices to publish in MQTT
// The discovery runs from time to time to capture new devices and detect ones
// that went innactive.

use echonet_lite as el;
use el::{prelude::*, props};
use std::net::{Ipv4Addr, UdpSocket};
use std::time::Duration;

use tracing::info;

use crate::echonet::profile::InstanceList;
use crate::error::Error;

use super::data::ProfilePropertyCode;

const EL_MULTICAST_ADDR: Ipv4Addr = Ipv4Addr::new(224, 0, 23, 0);

struct Discovery {}

impl super::Listener for Discovery {
    // do something with the incoming packet
    // returns a vectors of new discovery packets to send.
    fn process(_request: super::Request) -> Vec<super::Response> {
        vec![]
    }
}

pub fn find() -> Result<(), Error> {
    let socket = UdpSocket::bind("0.0.0.0:3610")?;
    socket.set_read_timeout(Some(Duration::from_secs(2)))?;
    socket.set_multicast_loop_v4(true)?;
    socket.join_multicast_v4(&EL_MULTICAST_ADDR, &[0, 0, 0, 0].into())?;

    let packet = el::ElPacketBuilder::new()
        .transaction_id(1)
        .seoj([0x05u8, 0xFFu8, 0x01u8])
        .deoj([0x0Eu8, 0xF0u8, 0x01u8])
        .esv(el::ServiceCode::Get)
        .props(el::props!(
            [ProfilePropertyCode::UniqueIdentifierData as u8, []],
            [ProfilePropertyCode::SelfNodeClasses as u8, []],
            [ProfilePropertyCode::SelfNodeClassListS as u8, []],
            [ProfilePropertyCode::SelfNodeInstances as u8, []],
            [ProfilePropertyCode::SelfNodeInstanceListS as u8, []]
        ))
        .build();
    let bytes = packet.serialize().expect("fail to serialize");

    socket.send_to(&bytes, (EL_MULTICAST_ADDR, 3610))?;
    loop {
        let mut buffer = [0u8; 1024];
        match socket.recv_from(&mut buffer) {
            Err(_) => break,
            Ok((_, src_addr)) => {
                if let Ok((_, response)) = el::ElPacket::from_bytes(&buffer) {
                    if response.is_response_for(&packet) {
                        let obj: ClassPacket = response.clone().into();
                        info!("got response from {}", src_addr);
                        info!("{:}", obj);
                        if let ClassPacket::Profile(_) = obj {
                            let instances = InstanceList::from(response.props)?;
                            for instance in instances {
                                let obj = ClassPacket::new(instance, props![]);
                                info!("  * instance: {obj}")
                            }
                        }
                    }
                }
            }
        }
    }

    Ok(())
}
