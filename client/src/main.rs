use std::{str, thread};
use std::cmp::min;
use std::cmp::Reverse;
use std::convert::TryInto;
use std::io::Write;
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::Duration;

use binary_heap_plus::*;
use scrap::{Capturer, Display};
use threadpool::ThreadPool;

const DESTINATION: &str = "3.14.82.97:8080";

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8081").unwrap();

    let cloned = socket.try_clone().unwrap();
    thread::spawn(move || {
        loop {
            let mut buf = [0 as u8; 32];
            let (_, src) = cloned.recv_from(&mut buf).unwrap();
            println!("{:?}, {:?}", buf, src);
        }
    });

    loop {
        socket.send_to(&[], DESTINATION).unwrap();
        sleep(Duration::from_millis(3000));
    }
}