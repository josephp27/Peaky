use std::net::UdpSocket;
use std::str;

use threadpool::ThreadPool;

const BUFFER_SIZE: usize = 2048;
const NUM_THREADS: usize = 20;

fn main() {
    const ADDRESS: &str = "0.0.0.0:8080";
    println!("listening on: {}", ADDRESS);

    let socket: UdpSocket = UdpSocket::bind(ADDRESS).unwrap();
    let pool = ThreadPool::new(NUM_THREADS);

    for _ in 0..NUM_THREADS {
        let cloned = socket.try_clone().unwrap();
        pool.execute(move || {
            let mut buffer = [0 as u8; BUFFER_SIZE];
            loop {
                let (amt, src) = cloned.recv_from(&mut buffer).unwrap();
                cloned.send_to(&buffer[..amt], src).unwrap();
            }
        });
    }
    pool.join();
}
