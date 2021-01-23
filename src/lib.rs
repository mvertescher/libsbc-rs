//! Bindings to the Linux Bluetooth low-complexity, subband codec (SBC) library.

extern crate failure;
extern crate libsbc_sys as ffi;
extern crate slice_deque;

pub use crate::error::{Error, ErrorKind, Result};

mod error;

use std::io::Read;
use std::mem;

use failure::ResultExt;
use slice_deque::SliceDeque;

// TODO: Tune these buffer sizes
/// Maximum number of samples present in an SBC frame
const MAX_SAMPLES_PER_FRAME: usize = 8196;

const BUFFER_SIZE: usize = MAX_SAMPLES_PER_FRAME * 15;
const REFILL_TRIGGER: usize = MAX_SAMPLES_PER_FRAME * 8;

/// SBC stream decoder that produces frames.
pub struct Decoder<R>
where
    R: Read,
{
    reader: R,
    buffer: SliceDeque<u8>,
    sbc: Box<ffi::sbc_struct>,
}

unsafe impl<R> Send for Decoder<R>
where
        R: Read + Send,
{
}

/// A SBC frame
pub struct Frame {
    /// Decoded audio
    pub data: Vec<i16>,
    /// Sample rate in hertz.
    pub sample_rate: i32,
    /// Number of channels in the frame.
    pub channels: usize,
}

impl<R> Decoder<R>
where
    R: Read,
{

    /// Create a new decoder from the reader.
    pub fn new(reader: R) -> Decoder<R> {
        let mut sbc = unsafe { Box::new(mem::zeroed()) };
        unsafe {
            // TODO: Magic number
            ffi::sbc_init(&mut *sbc, 0);
            // sbc.endian = ffi::SBC_BE as u8;
        };

        Decoder {
            reader,
            buffer: SliceDeque::with_capacity(BUFFER_SIZE),
            sbc,
        }
    }

    /// Decode the next frame from the stream.
    pub fn next_frame(&mut self) -> Result<Frame> {
        loop {
            if self.buffer.len() < REFILL_TRIGGER {
                if self.refill()? == 0 {
                    return Err(Error::eof());
                }
            }

            return self.decode_frame()
        }
    }

    fn decode_frame(&mut self) -> Result<Frame> {
        let mut pcm: Vec<i16> = Vec::with_capacity(MAX_SAMPLES_PER_FRAME);

        let mut num_written: usize = 0;
        let num_read: isize = unsafe {
            ffi::sbc_decode(
                &mut *self.sbc,
                self.buffer.as_ptr() as *const std::os::raw::c_void,
                self.buffer.len(),
                pcm.as_mut_ptr() as *mut std::os::raw::c_void,
                pcm.capacity(),
                &mut num_written,
            ) as _
        };

        let sample_rate = match self.sbc.frequency as u32 {
            ffi::SBC_FREQ_16000 => 16000,
            ffi::SBC_FREQ_32000 => 32000,
            ffi::SBC_FREQ_44100 => 44100,
            ffi::SBC_FREQ_48000 => 48000,
            _ => return Err(ErrorKind::BadDecode.into()),
        };

        let channels = match self.sbc.mode as u32 {
            ffi::SBC_MODE_MONO => 1,
            ffi::SBC_MODE_DUAL_CHANNEL => 2,
            ffi::SBC_MODE_STEREO => 2,
            ffi::SBC_MODE_JOINT_STEREO => 2,
            _ => return Err(ErrorKind::BadDecode.into()),
        };

        if num_written > 0 {
            // Divide by the size of i16
            unsafe { pcm.set_len(num_written / 2) }
        }

        let frame = Frame {
            data: pcm,
            sample_rate,
            channels,
        };

        let current_len = self.buffer.len();
        if num_read < 0 || num_read as usize > current_len {
            return Err(ErrorKind::BadDecode.into());
        }
        let num_read = num_read as usize;

        self.buffer.truncate_front(current_len - num_read);

        if num_written == 0 {
            Err(ErrorKind::NoData.into())
        } else {
            Ok(frame)
        }
    }

    fn refill(&mut self) -> Result<usize> {
        let mut data: [u8; MAX_SAMPLES_PER_FRAME * 5] = [0; MAX_SAMPLES_PER_FRAME * 5];
        let bytes_read = self.reader.read(&mut data).context(ErrorKind::Io)?;
        self.buffer.extend(data.iter());
        Ok(bytes_read)
    }
}
