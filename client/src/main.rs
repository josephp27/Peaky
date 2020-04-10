use std::io::{BufWriter, Read, Write};
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

fn display() {
    let ffplay = Command::new("ffplay")
        .args(&[
            "-analyzeduration", "100",
            "-fflags", "nobuffer",
            "-f", "mpegts",
            "-alwaysontop",
            "-"
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");
    let mut input = ffplay.stdin.unwrap();

    let socket = UdpSocket::bind("0.0.0.0:8082").unwrap();
    socket.send_to("start".as_bytes(), "18.220.60.51:8080").unwrap();

    let mut a = [0 as u8; 1024];
    loop {
        let (amt, _) = socket.recv_from(&mut a).unwrap();
        input.write_all(&a).unwrap();
    }
}

fn main() {
    let mut ffmpeg = Command::new("ffmpeg")
        .args(&[
            "-f", "avfoundation",
            "-i", "1",
            "-vf", "scale=256:128",
            "-pixel_format", "brg0",
            "-f", "mpegts",
            "-"
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");
    let mut output = ffmpeg.stdout.unwrap();

    let socket = UdpSocket::bind("0.0.0.0:8081").unwrap();
    let cloned_socket = socket.try_clone().unwrap();
    thread::spawn(move || display());

    let mut a = [0 as u8; 1024];
    loop {
        output.read(&mut a).unwrap();
        cloned_socket.send_to(&a, "18.220.60.51:8080").unwrap();
    }
}