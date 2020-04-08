use std::collections::HashSet;
use std::net::UdpSocket;
use std::thread;

fn main() {
    let mut buf = [0; 10];

    let socket = UdpSocket::bind("127.0.0.1:34254").unwrap();
    let mut clients = HashSet::new();
    loop {
        match socket.recv_from(&mut buf) {
            Ok((amt, src)) => {
                clients.insert(src);

                for dest in clients.clone() {
                    let cloned_socket = socket.try_clone().unwrap();

                    if src != dest {
                        thread::spawn(move || {
                            cloned_socket.send_to(&mut buf[..amt], &dest).unwrap();
                        });
                    }
                }
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}