use std::io::{Read, Write};
use std::net::SocketAddr;
use std::net::UdpSocket;
use std::process::{Command, Stdio};
use std::thread;

use socket2::{Domain, Protocol, Socket, Type};

const WIDTH: usize = 2560 / 8;
const HEIGHT: usize = 1440 / 8;
const BUFFER_SIZE: usize = WIDTH * HEIGHT;

fn display(socket: Socket) {
    let socky = Socket::new(Domain::ipv4(), Type::dgram(), Option::from(Protocol::udp())).unwrap();
    socky.bind(&"0.0.0.0:8084".parse::<SocketAddr>().unwrap().into()).unwrap();
    socky.set_recv_buffer_size(BUFFER_SIZE * 2);
    socky.send_to("start".as_bytes(), &"18.188.172.124:8080".parse::<SocketAddr>().unwrap().into());

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
    let mut a = [0 as u8; BUFFER_SIZE];

    loop {
        let (_, _) = socky.recv_from(&mut a).unwrap();
        input.write_all(&a).unwrap();
    }
}

fn main() {
    let mut ffmpeg = Command::new("ffmpeg")
        .args(&[
            "-f", "avfoundation",
            "-i", "1",
            "-vf", format!("scale={}:{}", WIDTH, HEIGHT).to_string().as_str(),
            "-pixel_format", "brg0",
            "-f", "mpegts",
            "-"
        ])
        .stdout(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");
    let mut output = ffmpeg.stdout.unwrap();

    let socket = Socket::new(Domain::ipv4(), Type::dgram(), Option::from(Protocol::udp())).unwrap();
    socket.bind(&"0.0.0.0:8081".parse::<SocketAddr>().unwrap().into()).unwrap();
    socket.set_send_buffer_size(BUFFER_SIZE * 2);

    let cloned_socket = socket.try_clone().unwrap();

    thread::spawn(move || display(socket));

    let mut a = [0 as u8; BUFFER_SIZE];
    loop {
        output.read(&mut a).unwrap();
        cloned_socket.send_to(&a, &"18.188.172.124:8080".parse::<SocketAddr>().unwrap().into()).unwrap();
    }
}