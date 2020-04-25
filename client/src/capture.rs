use std::cmp::min;
use std::iter::Iterator;
use std::net::UdpSocket;
use std::thread::sleep;
use std::time::Duration;

use scrap::{Capturer, Display};

pub fn capture(primary_display: Display, socket: UdpSocket, buffer_size: usize, destination: &str) {
    let mut queue_num = 0;
    let mut capturer = Capturer::new(primary_display).unwrap();

    loop {
        if let Ok(frame) = capturer.frame() {
            let mut packet_sequence: u32 = 0;
            for i in (0..frame.len()).step_by(buffer_size) {
                let end = i + min(buffer_size, frame.len() - i);

                let mut buffer = packet_sequence.to_be_bytes().to_vec();

                let data: &[u8] = &frame[i..end];

                buffer.append(&mut vec![queue_num]);
                buffer.append(&mut data.to_vec());

                socket.send_to(&buffer, destination).unwrap();

                packet_sequence += 1;
            }
            sleep(Duration::from_millis(100));
            queue_num ^= 1;
        }
    }
}