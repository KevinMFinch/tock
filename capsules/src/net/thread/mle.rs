use crate::net::ipv6::ip_utils::IPAddr;
use crate::net::udp::udp_recv::{UDPReceiver, UDPRecvClient};
use crate::net::udp::udp_send::{UDPSendClient, UDPSender};
use super::command::{self, Command};

use kernel::{debug, ReturnCode, Driver};

pub const DST_ADDR: IPAddr =     IPAddr([
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e,
        0x1f,
        // 0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e,
        // 0x0f,
]);
pub const SRC_PORT: u16 = 15123;
pub const DST_PORT: u16 = 16123;
pub const PAYLOAD_LEN: usize = 192;
const UDP_HDR_SIZE: usize = 8;

const COMMAND_WIDTH: usize = 2; // Number of bytes that the command type takes up (including the one byte for security info) 
const TL_WIDTH: usize = 2;


pub struct MLEClient<'a> {
    /// UDP sender
    udp_sender: &'a UDPSender<'a>,

    /// UDP receiver
    udp_receiver: &'a UDPReceiver<'a>,
}

impl<'a> MLEClient<'a> {
    pub fn new(
        sender: &'a UDPSender<'a>,
        receiver: &'a UDPReceiver<'a>,
    ) -> MLEClient<'a> {
        MLEClient {
            udp_sender: sender,
            udp_receiver: receiver,
        }
    }

    pub fn start_handshake(&self) {
        debug!("In start handshake");
       //debug!("{}", "In start handhsake");
        const len_buf: usize = COMMAND_WIDTH + (TL_WIDTH * 4) + 12; 
        let mut buf: [u8; len_buf] = [0; len_buf];
        //debug!("Starting to output buffer");
        command::format_command(Command::ParentRequest, &mut buf);
        for i in 0..buf.len() {
            //debug!("Buf[{}]: {}", i, buf[i]);
        }

        //debug!("Done with outputting buf");

        debug!("Sending message...");
        self.udp_sender.send_to(DST_ADDR, DST_PORT, SRC_PORT, &buf);
        /*unsafe {
            for i in 0..buf.len() {
                UDP_DGRAM[i] = buf[i];
            }

            debug!("Beginning send");
            self.udp_sender.send_to(DST_ADDR, DST_PORT, SRC_PORT, &UDP_DGRAM);
        }*/       
    }
}

impl<'a> UDPSendClient for MLEClient<'a> {
    fn send_done(&self, result: ReturnCode) {
        debug!("Sent through MLEClient");
    }
}

impl<'a> UDPRecvClient for MLEClient<'a> {
    fn receive(
        &self,
        src_addr: IPAddr,
        dst_addr: IPAddr,
        src_port: u16,
        dst_port: u16,
        payload: &[u8],
    ) {
        debug!("Receive in MLEClient");
    }
}