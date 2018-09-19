//! Play a SBC file via Alsa

extern crate alsa;
extern crate byteorder;
extern crate libsbc;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        println!("usage: {} <input sbc>", args[0]);
        return;
    }
    let input = &args[1];

    let file = std::fs::File::open(input).expect("Could not open input file");
    let mut decoder = libsbc::Decoder::new(file);
    let frame = decoder.next_frame().unwrap();

    let card = "default";
    let pcm = alsa::pcm::PCM::new(card, alsa::Direction::Playback, false).unwrap();
    let hwp = alsa::pcm::HwParams::any(&pcm).unwrap();
    hwp.set_channels(frame.channels as u32).unwrap();
    hwp.set_rate(frame.sample_rate as u32, alsa::ValueOr::Nearest).unwrap();
    hwp.set_format(alsa::pcm::Format::s16()).unwrap();
    hwp.set_access(alsa::pcm::Access::RWInterleaved).unwrap();
    pcm.hw_params(&hwp).unwrap();
    let io = pcm.io_i16().unwrap();

    let hwp = pcm.hw_params_current().unwrap();
    println!("hwp: {:?}", hwp);
    let swp = pcm.sw_params_current().unwrap();
    println!("swp: {:?}", swp);

    swp.set_start_threshold(hwp.get_buffer_size().unwrap() - hwp.get_period_size().unwrap()).unwrap();
    pcm.sw_params(&swp).unwrap();

    let mut num_frames = 0;
    loop {
        match decoder.next_frame() {
            Ok(f) => {
                io.writei(&f.data).expect("failed to write");
                num_frames += 1;
            }
            Err(e) => {
                match e.kind() {
                   libsbc::ErrorKind::Eof => break,
                   _ => panic!("unexpected error"),
                }
            }
        }
    }
    println!("Processed {} frames...", num_frames);
    // Wait for the stream to finish playback.
    pcm.drain().unwrap()
}
