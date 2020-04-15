use std::net::UdpSocket;
use std::str;

use threadpool::ThreadPool;

const BUFFER_SIZE: usize = 502;
const NUM_THREADS: usize = 32;

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
                let (_, src) = cloned.recv_from(&mut buffer).unwrap();
                if str::from_utf8(&buffer).unwrap().contains("done") {
                    println!("done");
                }
                cloned.send_to(&buffer, src).unwrap();
            }
        });
    }
    pool.join();
}
