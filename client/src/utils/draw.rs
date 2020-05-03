use std::sync::mpsc::Receiver;

use crate::utils::helper::get_canvas;
use crate::utils::settings::Settings;

pub fn draw(settings: Settings, rx: Receiver<Vec<u8>>) {
    let canvas = get_canvas(settings.width, settings.height, settings.scalar);
    loop {
        let data = rx.recv().unwrap();
        println!("Got: {:?}", data);
    }
}