use std::net::UdpSocket;
use std::thread;

use crate::utils::constants::NUM_THREADS;
use crate::utils::draw::draw;
use crate::utils::listener::listen;
use crate::utils::settings::Settings;
use std::sync::mpsc;

pub fn display_orchestrator(socket: UdpSocket, settings: Settings) {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || draw(settings, rx));

    (0..NUM_THREADS).for_each(|_| {
        let tx = tx.clone();
        let socket = socket.try_clone().unwrap();
        thread::spawn(move || listen(socket, tx));
    });
}
