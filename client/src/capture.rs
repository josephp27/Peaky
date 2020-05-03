
use scrap::{Capturer, Display};

use crate::utils::constants::{BUFFER_SIZE, DESTINATION};
use crate::utils::helper::{build_buffer, calculate_frame_end};
use std::net::UdpSocket;

pub fn capture_orchestrator(primary_display: Display, socket: UdpSocket) {
    let mut queue_num = 0;
    let mut capturer = Capturer::new(primary_display).unwrap();

    loop {
        if let Ok(frame) = capturer.frame() {
            let mut packet_sequence: u32 = 0;
            for i in (0..frame.len()).step_by(BUFFER_SIZE) {
                let end = calculate_frame_end(i, frame.len());
                let buffer = build_buffer(packet_sequence, &frame[i..end], queue_num);

                socket.send_to(&buffer, DESTINATION).unwrap();
                packet_sequence += 1;
            }
        }
    }
}