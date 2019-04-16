use crate::net::thread::tlv::Tlv;
use crate::net::thread::tlv::TlvType;
use crate::net::stream::SResult;

use kernel::{debug, ReturnCode};

const COMMAND_WIDTH: usize = 2; // Number of bytes that the command type takes up (including the one byte for security info) 
const TL_WIDTH: usize = 2; // Type and length fields of TLV are each one byte.
const MAX_VALUE_FIELD_LENGTH: usize = 128; // Assume a TLV value will be no longer than 128 bytes.

pub enum Command {
    LinkRequest = 0,
    LinkAccept = 1,
    LinkAcceptRequest = 2,
    LinkReject = 3,
    Advertisement = 4,
    /* Not used in Thread networks
    Update = 5,
    UpdateRequest = 6
    */
    DataRequest = 7,
    DataResponse = 8,
    ParentRequest = 9,
    ParentResponse = 10,
    ChildIdRequest = 11,
    ChildIdResponse = 12,
    ChildUpdateRequest = 13,
    ChildUpdateResponse = 14,
    Announce = 15,
    DiscoveryRequest = 16,
    DiscoveryResponse = 17,
}


pub fn format_command(command: Command, buf: &mut [u8]) {
    match command {
        Command::ParentRequest => {
            // 4 tlvs: Mode, Challenge, Scan Mask, Version
            // Lengths of each value are 1 byte, 8 bytes, 1 byte, 2 bytes, respectively
            // 12 total bytes for the values of these TLVs 
            
            buf[0] = 255 as u8;
            buf[1] = command as u8;

            let mode_tlv = Tlv::Mode(0);
            // TODO: Replace with random value
            let mut challenge: [u8; 8] = [1, 2, 3, 5, 123, 23, 67, 91];
            let challenge_tlv = Tlv::Challenge(challenge);
            let scanmask_tlv = Tlv::ScanMask(0b10000000 as u8); // Active routers must respond, REEDs not yet
            let version_tlv = Tlv::Version(2 as u16); // Version TLV contains '2' for Thread 1.1 (Section 4.5.19)

            mode_tlv.encode(&mut buf[2..]);
            challenge_tlv.encode(&mut buf[5..]);
            scanmask_tlv.encode(&mut buf[15..]);
            version_tlv.encode(&mut buf[18..]);
        }
        _ => debug!("Some other type of command matched"),
    }


    // let tlv: Tlv = Tlv::SourceAddress(257 as u16);
    // tlv.encode(&mut arr);
    // debug!("Len of array: {}", arr.len());
    // for x in 0..arr.len() {
    //     debug!("{}", arr[x]);
    // }


    // let res = Tlv::decode(&arr).done();
}


// Parent request has 4 TLVs (Mode, Challenge, Scan Mask, Version)

// Parent Response has 9 TLVs:
//    (Source Addr, Leader Data, Link Layer Frame Counter, [MLE Frame Counter], Response, Challenge, Link Margin, Connectivity, Version)

// MLE Child ID Request has 10 max TLVs
//    (Response, LL Frame COunter, MLE Frame COunter, Mode, Timeout, Version, Address REgistration, TLV Request: Addr 16, Active Timestammp, Pending Timestamp)

// MLE Child ID Response has 8 TLVs
//    (Source Addr, LEader Data, Address16, Network Data, Route64, Addr Registration, Active Operational Dataset, Pending Operational Dataset)



fn start() {

}