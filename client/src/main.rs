use std::net::UdpSocket;
use std::str;

use scrap::Display;

mod display;
mod capture;
mod utils;

const BUFFER_SIZE: usize = 7200;
const DESTINATION: &str = "127.0.0.1:8080";
const RECEIVER: &str = "127.0.0.1:7777";


fn main() {
    let socket = UdpSocket::bind(RECEIVER).unwrap();
    let cloned_socket = socket.try_clone().unwrap();

    let primary_display = Display::primary().unwrap();

    display::display(socket, BUFFER_SIZE);
    capture::capture(primary_display, cloned_socket, BUFFER_SIZE, DESTINATION);
}