use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;

use scrap::{Capturer, Display};

use crate::utils::constants::{BUFFER_SIZE, DESTINATION};
use crate::utils::draw::draw;
use crate::utils::helper::{build_buffer, calculate_frame_end};
use crate::utils::settings::Settings;

extern crate rand;

use rand::Rng;

pub fn capture_orchestrator(primary_display: Display, settings: Settings) {
    let mut capturer = Capturer::new(primary_display).unwrap();
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        draw(settings, rx);
    });
    let mut rng = rand::thread_rng();

    loop {
        if let Ok(frame) = capturer.frame() {
            let mut packet_sequence: u32 = 0;
            for i in (0..frame.len()).step_by(BUFFER_SIZE) {
                let val = rng.gen_range(1, 10);
                if packet_sequence % val == 0 {
                    packet_sequence += 1;
                    continue;
                }
                let end = calculate_frame_end(i, frame.len());
                let buffer = build_buffer(packet_sequence, &frame[i..end]);

                // socket.send_to(&buffer, DESTINATION).unwrap();
                tx.send(buffer).unwrap();
                packet_sequence += 1;
            }
            tx.send("done".as_bytes().to_vec()).unwrap();
        }
    }
}