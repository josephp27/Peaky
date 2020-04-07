extern crate scrap;

use std::io::ErrorKind::WouldBlock;
use std::io::Write;
use std::process::{Command, Stdio};

use scrap::{Capturer, Display};

fn main() {

    let d = Display::primary().unwrap();
    let (w, h) = (d.width() / 4, d.height() / 4);

    let child = Command::new("ffplay")
        .args(&[
            "-f", "rawvideo",
            "-alwaysontop",
            "-pixel_format", "bgr0",
            "-video_size", &format!("{}x{}", w, h),
            "-framerate", "60",
            "-"
        ])
        .stdin(Stdio::piped())
        .spawn()
        .expect("This example requires ffplay.");

    let mut capturer = Capturer::new(d).unwrap();
    let mut out = child.stdin.unwrap();

    loop {
        match capturer.frame() {
            Ok(frame) => {
                // Write the frame, removing end-of-row padding.
                let stride = frame.len() / h;
                let rowlen = 4 * w;

                println!("{:?}", frame.chunks(stride));
                for row in frame.chunks(stride) {
                    let row = &row[..rowlen];
                    out.write_all(row).unwrap();
                }
                break;
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