extern crate captrs;

use std::io::ErrorKind::WouldBlock;
use std::io::Write;
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use captrs::{Bgr8, Capturer};


fn display(socket: UdpSocket, w: u32, h: u32) {
    let socket = UdpSocket::bind("0.0.0.0:8082").unwrap();
    let child = Command::new("ffplay")
        .args(&[
            "-f", "rawvideo",
            "-alwaysontop",
            "-video_size", &format!("{}x{}", w, h),
            "-framerate", "60",
            "-"
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");

    let mut out = child.stdin.unwrap();

    let size = 8388608;
    let mut buf: Box<[u8]> = vec![0; size].into_boxed_slice();

    socket.send_to("start".as_bytes(), "18.220.60.51:8080").unwrap();
    loop {
        let (amt, _) = socket.recv_from(&mut buf).unwrap();
        let buf = &mut buf[..amt];
        out.write_all(buf).unwrap();
    }
}

fn main() {
    let mut capturer = Capturer::new(0).unwrap();

    let (w, h) = capturer.geometry();
    let size = w as u64 * h as u64;

    // let socket = UdpSocket::bind("0.0.0.0:8081").unwrap();
    // let cloned_socket = socket.try_clone().unwrap();
    // thread::spawn(move || display(socket.try_clone().unwrap(), w, h));


    loop {
        let ps = capturer.capture_frame().unwrap();

        let mut a = Vec::new();
        for Bgr8 { r, .. } in ps.into_iter() {
            a.push(r)
        }

        println!("{}", a.len());

        // cloned_socket.send_to(&row[..4 * w], "18.220.60.51:8080").unwrap();
    }
}