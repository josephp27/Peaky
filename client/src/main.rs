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
const LAPTOP: &str = "45.19.26.246:8081";

fn main() {
    let socket = UdpSocket::bind("0.0.0.0:8090").unwrap();
    socket.send_to(&[], DESTINATION).unwrap();
    socket.send_to(&[], LAPTOP).unwrap();

    loop {
        let mut buf = [0 as u8; 32];
        let (_, src) = socket.recv_from(&mut buf).unwrap();
        println!("{:?}, {:?}", buf, src);
    }
}