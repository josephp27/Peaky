use std::net::UdpSocket;

use scrap::Display;

use crate::utils::constants::RECEIVER;
use crate::utils::settings::Settings;

mod display;
mod capture;
mod utils;


fn main() {
    let socket = UdpSocket::bind(RECEIVER).unwrap();
    let cloned_socket = socket.try_clone().unwrap();

    let primary_display = Display::primary().unwrap();
    let settings = Settings::new(primary_display.width(),
                                 primary_display.height(), None);

    display::display_orchestrator(socket, settings);
    capture::capture(primary_display, cloned_socket);
}