use std::io::{BufWriter, Read, Write};
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

const WIDTH: usize = 2560 / 8;
const HEIGHT: usize = 1440 / 8;
const BUFFER_SIZE: usize = WIDTH * HEIGHT / 2;

fn display() {
    let ffplay = Command::new("ffplay")
        .args(&[
            "-analyzeduration", "100",
            "-fflags", "nobuffer",
            "-f", "mpegts",
            "-alwaysontop",
            "-probesize", "320",
            "-sync", "ext",
            "-"
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");
    let mut input = ffplay.stdin.unwrap();

    let socket = UdpSocket::bind("0.0.0.0:8085").unwrap();
    socket.send_to("start".as_bytes(), "18.220.60.51:8080").unwrap();

    let mut a = [0 as u8; BUFFER_SIZE];
    loop {
        let (amt, _) = socket.recv_from(&mut a).unwrap();
        input.write_all(&a).unwrap();
    }
}

fn main() {

    let mut ffmpeg = Command::new("ffmpeg")
        .args(&[
            "-f", "dshow",
            "-i", "video=screen-capture-recorder",
            "-vf", format!("scale={}:{}", WIDTH, HEIGHT).to_string().as_str(),
            "-pixel_format", "brg0",
            "-f", "mpegts",
            "-"
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");
    let mut output = ffmpeg.stdout.unwrap();

    let socket = UdpSocket::bind("0.0.0.0:8084").unwrap();
    let cloned_socket = socket.try_clone().unwrap();
    thread::spawn(move || display());

    let mut a = [0 as u8; BUFFER_SIZE];
    loop {
        output.read(&mut a).unwrap();
        cloned_socket.send_to(&a, "18.220.60.51:8080").unwrap();
    }
}