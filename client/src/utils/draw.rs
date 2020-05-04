use std::io::Write;
use std::sync::mpsc::Receiver;

use crate::utils::constants::BUFFER_SIZE;
use crate::utils::helper;
use crate::utils::helper::get_canvas;
use crate::utils::settings::Settings;

pub fn draw(settings: Settings, rx: Receiver<Vec<u8>>) {
    let mut canvas = get_canvas(settings.width, settings.height, settings.scalar);

    let frame_size = (settings.height * settings.width * 4 / BUFFER_SIZE) + 1;
    let mut buffer: Vec<Vec<u8>> = vec![vec![]; frame_size];
    loop {
        let data = rx.recv().unwrap();
        let index = helper::get_packet_num(&data[..4]);

        if buffer.get(index).unwrap().len() > 0 {
            let mut prev: Vec<u8> = vec![0; BUFFER_SIZE];
            for packet in buffer {
                if packet.len() == 0 {
                    canvas.write_all(&prev).unwrap()
                } else {
                    canvas.write_all(&packet).unwrap();
                    prev = packet;
                }
            }
            buffer = vec![vec![]; frame_size];
        } else {
            buffer[index] = data[4..].to_vec();
        }
    }
}