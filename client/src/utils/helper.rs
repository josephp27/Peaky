use std::cmp::min;
use std::convert::TryInto;

use crate::utils::constants::BUFFER_SIZE;

pub fn get_packet_num(buffer: &[u8]) -> u32 {
    u32::from_be_bytes(buffer.try_into().expect(""))
}

pub fn build_buffer(packet_sequence: u32, frame: &[u8], queue_num: u8) -> Vec<u8> {
    let mut buffer = packet_sequence.to_be_bytes().to_vec();

    buffer.append(&mut vec![queue_num]);
    buffer.append(&mut frame.to_vec());

    buffer
}

pub fn calculate_frame_end(i: usize, frame_length: usize) -> usize {
    i + min(BUFFER_SIZE, frame_length - i)
}