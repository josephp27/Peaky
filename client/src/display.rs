use std::net::UdpSocket;
use std::thread;

use crate::utils::constants::NUM_THREADS;
use crate::utils::draw::draw;
use crate::utils::listener::listen;
use crate::utils::settings::Settings;

pub fn display_orchestrator(socket: UdpSocket, settings: Settings) {
    thread::spawn(move || draw(settings));

    (0..NUM_THREADS).for_each(|_| {
        let socket = socket.try_clone().unwrap();
        thread::spawn(move || listen(socket));
    });
}
