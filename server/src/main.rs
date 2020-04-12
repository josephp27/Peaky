use std::collections::HashSet;
use std::net::SocketAddr;
use std::thread;

use socket2::{Domain, Protocol, Socket, Type};

const WIDTH: usize = 2560 / 8;
const HEIGHT: usize = 1440 / 8;
const BUFFER_SIZE: usize = WIDTH * HEIGHT;

fn create_socket(addr: &str) -> Socket {
    let socket = Socket::new(Domain::ipv4(), Type::dgram(), Option::from(Protocol::udp())).unwrap();
    socket.bind(&addr.parse::<SocketAddr>().unwrap().into()).unwrap();
    socket.set_send_buffer_size(BUFFER_SIZE * 2);
    socket.set_recv_buffer_size(BUFFER_SIZE * 2);

    socket
}

fn main() {
    let addr = "0.0.0.0:8080";
    println!("listening on: {}", addr);

    let socket = create_socket(addr);
    let mut clients = HashSet::new();

    loop {
        let mut buffer = [0 as u8; BUFFER_SIZE];

        let (amt, src) = socket.recv_from(&mut buffer).unwrap();
        clients.insert(src.as_inet().unwrap().to_string());

        println!("{:?}", clients);
        for dest in clients.clone() {
            let cloned_socket = socket.try_clone().unwrap();
            let cloned_buffer = buffer.clone();

            if src.as_inet().unwrap().to_string() != dest.to_string() {
                thread::spawn(move || {
                    let addr = &dest.parse::<SocketAddr>().unwrap().into();
                    cloned_socket.send_to(&cloned_buffer[..amt], addr).unwrap();
                });
            }
        }
    }
}
