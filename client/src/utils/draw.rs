use std::io::Write;
use std::sync::mpsc::Receiver;

use crate::utils::constants::BUFFER_SIZE;
use crate::utils::helper;
use crate::utils::helper::get_canvas;
use crate::utils::settings::Settings;

pub fn draw(settings: Settings, rx: Receiver<Vec<u8>>) {
    let mut canvas = get_canvas(settings.width, settings.height, settings.scalar);

    let mut buffer: Vec<u8> = vec![0; 8028160];
    loop {
        let data = rx.recv().unwrap();

        if data.len() == 4 {
            canvas.write_all(&buffer);
        } else {
            let index = BUFFER_SIZE * helper::get_packet_num(&data[..4]);
            let mut vec1 = data[4..].to_vec();
            buffer.splice(index..index + vec1.len(), vec1);
        }
    }
}