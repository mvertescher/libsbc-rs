//! Decode a SBC file to raw file

extern crate libsbc;

use std::fs::File;
use std::io::Write;
use libsbc::{Error, ErrorKind};

fn main() {
    println!("decode a file");
    let name = "examples/test.sbc";
    let file = File::open(name).expect("Could not open file");

    let mut decoder = libsbc::Decoder::new(file);

    let output = File::create("aud.raw").expect("Failed to open output file");
    loop {
        match decoder.next_frame() {
            Ok(f) => {
                let bytes: Vec<u8> = f;
                output.write(&f).unwrap();
            }
            Err(e) => {
                match e {
                   Error(ErrorKind::Eof, _) => break,
                   Error(_, _) => panic!("wtf"),
                }
            }
        }
    }
}
