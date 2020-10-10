use std::io::Write;
use std::sync::mpsc::Receiver;
use std::thread;
use std::time::{Duration, Instant};

use crate::utils::constants::{BUFFER_SIZE, FRAME_MAX_RUNTIME};
use crate::utils::helper;
use crate::utils::helper::get_canvas;
use crate::utils::settings::Settings;

pub fn draw(settings: Settings, rx: Receiver<Vec<u8>>) {
    let mut canvas = get_canvas(settings.width, settings.height, settings.scalar);
    let mut scan_index = 0;

    let mut buffer: Vec<Vec<u8>> = vec![vec![0; BUFFER_SIZE]; 1116];
    loop {
        let data = rx.recv().unwrap();

        let index = helper::get_packet_num(&data[..4]);
        let mut vec1 = data[4..].to_vec();
        buffer[index] = vec1;

        canvas.write(&buffer[scan_index % buffer.len()]);

        scan_index += 1;
    }
}