use std::net::UdpSocket;
use std::sync::mpsc::Sender;

use crate::utils::constants::{BUFFER_SIZE, PACKET_NUM_SIZE};

pub fn listen(socket: UdpSocket, tx: Sender<Vec<u8>>) {
    loop {
        let mut buffer = vec![0 as u8; BUFFER_SIZE + PACKET_NUM_SIZE];
        let (amt, src) = socket.recv_from(&mut buffer).unwrap();

        tx.send(buffer[..amt].to_vec()).unwrap();
    }
}