use std::collections::HashSet;
use std::net::UdpSocket;
use std::thread;

fn main() {

    let addr = "0.0.0.0:8080";
    println!("listening on: {}", addr);

    let socket = UdpSocket::bind(addr).unwrap();
    let mut clients = HashSet::new();

    loop {
        let size = 8388608;
        let mut buffer2: Box<[u8]> = vec![0; size].into_boxed_slice();
        match socket.recv_from(&mut buffer2) {
            Ok((amt, src)) => {
                clients.insert(src);

                println!("{:?}", clients);
                for dest in clients.clone() {
                    let cloned_socket = socket.try_clone().unwrap();
                    let buffy = buffer2.clone();
                    if src != dest {
                        thread::spawn(move || {
                            cloned_socket.send_to(&buffy[..amt], &dest).unwrap();
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