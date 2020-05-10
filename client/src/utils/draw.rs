use std::io::Write;
use std::sync::mpsc::Receiver;

use crate::utils::constants::BUFFER_SIZE;
use crate::utils::helper;
use crate::utils::helper::get_canvas;
use crate::utils::settings::Settings;

pub fn draw(settings: Settings, rx: Receiver<Vec<u8>>) {
    let mut canvas = get_canvas(settings.width, settings.height, settings.scalar);

    let mut buffer: Vec<Vec<u8>> = vec![vec![0; BUFFER_SIZE]; 1116];
    loop {
        let data = rx.recv().unwrap();

        if data.len() == 4 {
            for packet in buffer.clone() {
                canvas.write_all(&packet);
            }
        } else {
            let index = helper::get_packet_num(&data[..4]);
            let mut vec1 = data[4..].to_vec();
            buffer[index] = vec1;
        }
    }
}