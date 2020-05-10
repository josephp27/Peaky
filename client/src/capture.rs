extern crate rand;

use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;

use rand::Rng;
use scrap::{Capturer, Display};

use crate::utils::constants::{BUFFER_SIZE, DESTINATION};
use crate::utils::draw::draw;
use crate::utils::helper::{build_buffer, calculate_frame_end};
use crate::utils::settings::Settings;


pub fn capture_orchestrator(primary_display: Display, socket: UdpSocket) {
    let mut capturer = Capturer::new(primary_display).unwrap();

    loop {
        if let Ok(frame) = capturer.frame() {
            let mut packet_sequence: u32 = 0;
            for i in (0..frame.len()).step_by(BUFFER_SIZE) {
                let end = calculate_frame_end(i, frame.len());
                let buffer = build_buffer(packet_sequence, &frame[i..end]);

                socket.send_to(&buffer, DESTINATION).unwrap();
                packet_sequence += 1;
            }
            socket.send_to(&"done".as_bytes().to_vec(), DESTINATION).unwrap();
        }
    }
}