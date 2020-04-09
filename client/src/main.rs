extern crate scrap;

use std::io::ErrorKind::WouldBlock;
use std::io::Write;
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use scrap::{Capturer, Display};

fn display(socket: UdpSocket, w: usize, h: usize) {
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
    let d = Display::primary().unwrap();
    let (w, h) = (d.width() / 2, d.height() / 2);

    let mut capturer = Capturer::new(d).unwrap();

    let socket = UdpSocket::bind("0.0.0.0:8081").unwrap();
    let cloned_socket = socket.try_clone().unwrap();
    thread::spawn(move || display(socket.try_clone().unwrap(), w, h));


    loop {
        match capturer.frame() {
            Ok(frame) => {
                // Write the frame, removing end-of-row padding.
                let stride = frame.len() / h;
                let rowlen = 4 * w;

                let mut red = [0; 819200];
                let mut j = 0;
                for row in frame.chunks(stride) {
                    for i in 0..rowlen {
                        if i % 2 == 0 {
                            red[j] = row[i];
                            j += 1;
                        }
                    }
                }
                cloned_socket.send_to(&red[..819200 / 1000], "18.220.60.51:8080").unwrap();
            }
            Err(ref e) if e.kind() == WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                // We're done here.
                break;
            }
        }
    }
}