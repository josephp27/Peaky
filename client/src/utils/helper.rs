use std::cmp::min;
use std::convert::TryInto;
use std::process::{ChildStdin, Command, Stdio};

use crate::utils::constants::BUFFER_SIZE;

pub fn get_packet_num(buffer: &[u8]) -> usize {
    u32::from_be_bytes(buffer.try_into().expect("")) as usize
}

pub fn build_buffer(packet_sequence: u32, frame: &[u8]) -> Vec<u8> {
    let mut buffer = packet_sequence.to_be_bytes().to_vec();
    buffer.append(&mut frame.to_vec());

    buffer
}

pub fn calculate_frame_end(i: usize, frame_length: usize) -> usize {
    i + min(BUFFER_SIZE, frame_length - i)
}

pub fn get_canvas(w: usize, h: usize, scalar: usize) -> ChildStdin {
    Command::new("ffplay")
        .args(&[
            "-f", "rawvideo",
            "-alwaysontop",
            "-fflags", "nobuffer",
            "-pixel_format", "bgr0",
            "-video_size", &format!("{}x{}", w, h),
            "-x", &format!("{}", w / scalar),
            "-y", &format!("{}", h / scalar),
            // "-framerate", "60",
            "-"
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.")
        .stdin.unwrap()
}