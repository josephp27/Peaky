extern crate rand;

use std::net::UdpSocket;
use std::os::raw::c_float;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

use rand::Rng;
use scrap::{Capturer, Display};

use crate::utils::constants::{BUFFER_SIZE, DESTINATION, FRAME_MAX_RUNTIME};
use crate::utils::draw::draw;
use crate::utils::helper::{build_buffer, calculate_frame_end};
use crate::utils::settings::Settings;

pub fn capture_orchestrator(primary_display: Display, socket: UdpSocket) {
    let mut capturer = Capturer::new(primary_display).unwrap();

    loop {
        let start = Instant::now();

        if let Ok(frame) = capturer.frame() {
            let mut packet_sequence: u32 = 0;

            for i in (0..frame.len()).step_by(BUFFER_SIZE) {
                let end = calculate_frame_end(i, frame.len());
                let buffer = build_buffer(packet_sequence, &frame[i..end]);

                socket.send_to(&buffer, DESTINATION).unwrap();
                packet_sequence += 1;
            }
        }

        let time_left = FRAME_MAX_RUNTIME - (start.elapsed().as_millis() as f64);
        let duration = time_left.max(0.);
        thread::sleep(Duration::from_millis((duration * 1000.) as u64));
    }
}