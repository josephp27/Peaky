extern crate priority_queue;

use std::cmp::min;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::convert::TryInto;
use std::io::Write;
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::str;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use priority_queue::PriorityQueue;
use scrap::{Capturer, Display};
use threadpool::ThreadPool;

const BUFFER_SIZE: usize = 502;
// const DESTINATION: &str = "18.188.172.124:8080";
const DESTINATION: &str = "0.0.0.0:8080";
const NUM_THREADS: usize = 10;

fn pop(barry: &[u8]) -> [u8; 4] {
    barry.try_into().expect("")
}

fn display(socket: UdpSocket) -> ThreadPool {
    let frame = Arc::new(Mutex::new(BinaryHeap::new()));
    let pool = ThreadPool::new(NUM_THREADS + 1);

    for _ in 0..NUM_THREADS {
        let socket = socket.try_clone().unwrap();
        let frame = frame.clone();

        pool.execute(move || {
            let mut a = vec![0 as u8; BUFFER_SIZE + 4];
            loop {
                let (_, _) = socket.recv_from(&mut a).unwrap();

                let packet_num = u32::from_be_bytes(pop(&a[..4]));
                let data = a[4..].to_vec();

                // println!("{}", packet_num);

                let mut guard = frame.lock().unwrap();
                guard.push((data, Reverse(packet_num)));
                // println!("{} {}", packet_num, guard.len());
            }
        });
    }

    pool.execute(move || {
        let child = Command::new("ffplay")
            .args(&[
                "-f", "rawvideo",
                "-alwaysontop",
                "-pixel_format", "bgr0",
                "-video_size", &format!("{}x{}", 1280, 800),
                "-framerate", "60",
                "-"
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("This example requires ffplay.");

        let mut out = child.stdin.unwrap();
        loop {
            // println!("{}", frame.lock().unwrap().len());
            if frame.lock().unwrap().len() >= 8100 {
                let mut a = frame.lock().unwrap();
                for _ in 0..8100 {
                    let (buf, num) = a.pop().unwrap();
                    out.write_all(&buf);
                }
            }
        }
    });

    pool
}

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8081").unwrap();
    let cloned_socket = socket.try_clone().unwrap();

    // display(socket);

    let d = Display::primary().unwrap();
    let mut capturer = Capturer::new(d).unwrap();

    loop {
        let mut packet_sequence: u32 = 0;
        if let Ok(frame) = capturer.frame() {
            for i in (0..frame.len()).step_by(BUFFER_SIZE) {
                let end = i + min(BUFFER_SIZE, frame.len() - i);

                let mut buffer = packet_sequence.to_be_bytes().to_vec();
                let data: &[u8] = &frame[i..end];
                buffer.append(&mut data.to_vec());

                cloned_socket.send_to(&buffer, DESTINATION).unwrap();

                out.write_all(&data);
                packet_sequence += 1;
            }
        }
    }
}