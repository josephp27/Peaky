use std::net::UdpSocket;

use scrap::Display;

use crate::utils::constants::RECEIVER;

mod display;
mod capture;
mod utils;


fn main() {
    let socket = UdpSocket::bind(RECEIVER).unwrap();
    let cloned_socket = socket.try_clone().unwrap();

    let primary_display = Display::primary().unwrap();

    display::display(socket);
    capture::capture(primary_display, cloned_socket);
}