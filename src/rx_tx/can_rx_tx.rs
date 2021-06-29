use ccsds::{
    c_packet::CANpacket,
    cuc_time
};

use socketCAN::{
    socketCAN,
    CANFrame
}

use util::du::*;
use std::Result;

pub mod CANCSP {

    pub <'a> trait CAN_RX_TX {

        // csp packets can be more than 8 bits
        // canframes are 16 bits but can only have 8 bits of information

        fn chop(packet: CANpacket) -> Result<Some(Vec<CanFrame>), Error>;                   // chops a csp packet into CANFrames to send
        fn unchop(frames: Vec<CanFrame>) -> Result<Some(CANpacket), Error>;                 // reassembles csp packet from can frames

        pub fn can_rx(socket: &socketCAN, packet: CANpacket) -> Result<Some(bool), Error>;  // sends a chopped csp packet through the can socket interface
        pub fn can_tx(socket: &socketCAN) -> Result<Some(CANpacket), Error>;                // recv a chopped csp packet and reassembles it

    }

    impl <'a> CAN_RX_TX for CANpacket<'a> { // implemenation of CANBUS csp interface

    }

}