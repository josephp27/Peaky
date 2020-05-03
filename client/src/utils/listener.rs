use std::net::UdpSocket;
use std::sync::mpsc::Sender;

use crate::utils::constants::{BUFFER_SIZE, PACKET_NUM_SIZE, QUEUE_NUM_SIZE};
use crate::utils::helper;

pub fn listen(socket: UdpSocket, tx: Sender<Vec<u8>>) {
    let mut a = vec![0 as u8; BUFFER_SIZE + PACKET_NUM_SIZE + QUEUE_NUM_SIZE];
    loop {
        let (_, _) = socket.recv_from(&mut a).unwrap();

        let packet_num = helper::get_packet_num(&a[..4]);
        let queue_num = a[4];
        let data = a[5..].to_vec();

        tx.send(data).unwrap();
    }
}