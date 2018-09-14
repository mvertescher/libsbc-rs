//! Decode a SBC file to raw file

extern crate libsbc;
extern crate byteorder;

use std::env;
use std::fs::File;
use std::io::Write;

use byteorder::{WriteBytesExt, LittleEndian};
use libsbc::{Error, ErrorKind};

fn main() {
    let args: Vec<String> = env::args().collect();

    let input = &args[1];
    let output = &args[2];

    let file = File::open(input).expect("Could not open input file");

    let mut decoder = libsbc::Decoder::new(file);

    let mut output = File::create(output).expect("Could not open output file");
    let mut num_frames = 0;
    loop {
        match decoder.next_frame() {
            Ok(f) => {
                output.write(&convert(f)).unwrap();
                num_frames += 1;
            }
            Err(e) => {
                match e {
                   Error(ErrorKind::Eof, _) => break,
                   Error(_, _) => panic!("wtf"),
                }
            }
        }
    }
    println!("Processed {} frames...", num_frames);
}

fn convert(input: Vec<i16>) -> Vec<u8> {
    let mut output: Vec<u8> = Vec::new();
    for n in input {
        output.write_i16::<LittleEndian>(n).unwrap();
    }
    output
}
