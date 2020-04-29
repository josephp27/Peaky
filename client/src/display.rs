use std::cmp::Reverse;
use std::io::Write;
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};

use binary_heap_plus::{BinaryHeap, KeyComparator};
use threadpool::ThreadPool;

use crate::utils::constants::{BUFFER_SIZE, PACKET_NUM_SIZE, QUEUE_NUM_SIZE};
use crate::utils::helper;

const NUM_THREADS: usize = 10;

pub fn display(socket: UdpSocket) -> ThreadPool {
    let k = KeyComparator(|k: &(u32, Vec<u8>)| Reverse(k.0));
    let k2 = KeyComparator(|j: &(u32, Vec<u8>)| Reverse(j.0));
    let frame = Arc::new(Mutex::new(BinaryHeap::from_vec_cmp(vec![], k)));
    let frame2 = Arc::new(Mutex::new(BinaryHeap::from_vec_cmp(vec![], k2)));
    let pool = ThreadPool::new(NUM_THREADS + 1);
    let curr_queue = Arc::new(Mutex::new(0));

    let frame_display = frame.clone();
    let frame_display2 = frame2.clone();
    let curr_queue_display = curr_queue.clone();
    pool.execute(move || {
        let child = Command::new("ffplay")
            .args(&[
                "-f", "rawvideo",
                // "-alwaysontop",
                "-fflags", "nobuffer",
                "-pixel_format", "bgr0",
                "-video_size", &format!("{}x{}", 2560, 1440),
                "-x", &format!("{}", 2560 / 4),
                "-y", &format!("{}", 1440 / 4),
                // "-framerate", "60",
                "-"
            ])
            .stdin(Stdio::piped())
            .spawn()
            .expect("This example requires ffplay.");

        let mut out = child.stdin.unwrap();
        loop {
            if *curr_queue_display.lock().unwrap() == 1 {
                let mut guard1 = frame_display.lock().unwrap();
                let len = guard1.len();

                for _ in 0..len {
                    out.write_all(&guard1.pop().unwrap().1).unwrap();
                }
            } else {
                let mut guard2 = frame_display2.lock().unwrap();
                let len = guard2.len();
                for _ in 0..len {
                    out.write_all(&guard2.pop().unwrap().1).unwrap();
                }
            }
        }
    });

    for _ in 0..NUM_THREADS {
        let socket = socket.try_clone().unwrap();
        let frame = frame.clone();
        let frame2 = frame2.clone();
        let curr_queue = curr_queue.clone();

        pool.execute(move || {
            let mut a = vec![0 as u8; BUFFER_SIZE + PACKET_NUM_SIZE + QUEUE_NUM_SIZE];
            loop {
                let (_, _) = socket.recv_from(&mut a).unwrap();

                let packet_num = helper::get_packet_num(&a[..4]);
                let queue_num = a[4];
                let data = a[5..].to_vec();
                *curr_queue.lock().unwrap() = queue_num;

                if queue_num == 1 {
                    let mut guard2 = frame2.lock().unwrap();
                    guard2.push((packet_num, data));
                } else {
                    let mut guard = frame.lock().unwrap();
                    guard.push((packet_num, data));
                }
            }
        });
    }


    pool
}
