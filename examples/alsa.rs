//! Play a SBC file via Alsa

extern crate alsa;
extern crate byteorder;
extern crate libsbc;

use std::io::Write;

use byteorder::{WriteBytesExt, LittleEndian};
use libsbc::{Error, ErrorKind};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let input = &args[1];

    let card = "default";
    let channels = 2;
    let rate = 44100;
    let format = alsa::pcm::Format::s16;

    let pcm = alsa::pcm::PCM::new(card, alsa::Direction::Playback, false).unwrap();
    let hwp = alsa::pcm::HwParams::any(&pcm).unwrap();
    hwp.set_channels(channels).unwrap();
    hwp.set_rate(rate, alsa::ValueOr::Nearest).unwrap();
    hwp.set_format(alsa::pcm::Format::s16()).unwrap();
    hwp.set_access(alsa::pcm::Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    let mut io = pcm.io_i16().unwrap();

    let hwp = pcm.hw_params_current().unwrap();
    println!("hwp: {:?}", hwp);
    let swp = pcm.sw_params_current().unwrap();
    println!("swp: {:?}", swp);

    swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
    pcm.sw_params(&swp).unwrap();


    let file = std::fs::File::open(input).expect("Could not open input file");
    let mut decoder = libsbc::Decoder::new(file);

    let mut num_frames = 0;
    loop {
        match decoder.next_frame() {
            Ok(f) => {
                io.writei(&f).expect("failed to write");
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

