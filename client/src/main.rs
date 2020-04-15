use std::net::UdpSocket;
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use threadpool::ThreadPool;

const BUFFER_SIZE: usize = 502;
// const DESTINATION: &str = "18.188.172.124:8080";
const DESTINATION: &str = "0.0.0.0:8080";
const NUM_THREADS: usize = 10;

fn display(socket: UdpSocket) -> ThreadPool {
    let pool = ThreadPool::new(20);
    for _ in 0..20 {
        let socket = socket.try_clone().unwrap();
        pool.execute(move || {
            let mut a = [0 as u8; BUFFER_SIZE];
            loop {
                let (_, _) = socket.recv_from(&mut a).unwrap();
                println!("received");
            }
        });
    }

    pool
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8081").unwrap();
    let cloned_socket = socket.try_clone().unwrap();

    let pool = display(socket);

    for _ in 0..40000 {
        cloned_socket.send_to("send".as_bytes(), DESTINATION).unwrap();
    }

    pool.join();
}