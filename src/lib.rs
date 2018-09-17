//! Bindings to the Linux Bluetooth low-complexity, subband codec (SBC) library.

extern crate libsbc_sys as ffi;
extern crate slice_deque;
#[macro_use] extern crate error_chain;

use std::io::{self, Read};
use std::mem;

use slice_deque::SliceDeque;

error_chain! {
    foreign_links {
        Io(std::io::Error);
    }

    errors {
        Eof {
            description("end of file")
            display("end of file")
        }

        NoData {
            description("out of data")
            display("out of data")
        }
    }
}

// TODO: Not sure if this is correct
/// Maximum number of samples present in an SBC frame
const MAX_SAMPLES_PER_FRAME: usize = 8196;

const BUFFER_SIZE: usize = MAX_SAMPLES_PER_FRAME * 15;
const REFILL_TRIGGER: usize = MAX_SAMPLES_PER_FRAME * 8;

// TODO: Maybe:
// pub struct Frame(Vec<i16>)

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
        R: Read,
{
}

impl<R> Decoder<R>
where
    R: Read, // maybe: seek + send
{

    /// Create a new decoder from the reader.
    pub fn new(reader: R) -> Decoder<R> {
        let mut sbc = unsafe { Box::new(mem::zeroed()) };
        unsafe {
            // TODO: Magic number
            ffi::sbc_init(&mut *sbc, 1);
            // sbc.endian = ffi::SBC_BE as u8;
        };

        Decoder {
            reader,
            buffer: SliceDeque::with_capacity(BUFFER_SIZE),
            sbc,
        }
    }

    /// Decode the next frame from the stream.
    pub fn next_frame(&mut self) -> Result<Vec<i16>> {
        if self.buffer.len() < REFILL_TRIGGER {
            if self.refill()? == 0 {
                return Err(ErrorKind::Eof.into());
            }
        }

        self.decode_frame()
    }

    fn decode_frame(&mut self) -> Result<Vec<i16>> {
        let mut pcm: Vec<i16> = Vec::with_capacity(MAX_SAMPLES_PER_FRAME);

        let mut num_written: usize = 0;
        let num_read: usize = unsafe {
            ffi::sbc_decode(
                &mut *self.sbc,
                self.buffer.as_ptr() as *const std::os::raw::c_void,
                self.buffer.len() as _,
                pcm.as_mut_ptr() as *mut std::os::raw::c_void,
                pcm.capacity() as _,
                &mut num_written, //  as *mut _,
            ) as _
        };

        if num_written > 0 {
            // Divide by the size of i16
            unsafe { pcm.set_len(num_written / 2) }
        }

        let current_len = self.buffer.len();
        self.buffer.truncate_front(current_len - num_read);

        if num_written == 0 {
            Err(ErrorKind::NoData.into())
        } else {
            Ok(pcm)
        }
    }

    fn refill(&mut self) -> Result<usize> {
        let mut data: [u8; MAX_SAMPLES_PER_FRAME * 5] = [0; MAX_SAMPLES_PER_FRAME * 5];
        let bytes_read = self.reader.read(&mut data)?;
        self.buffer.extend(data.iter());
        Ok(bytes_read)
    }


/*
    pub fn info(&mut self) {
        // TODO: This.
        let info_str = unsafe { ffi::sbc_get_implementation_info(&mut *self.sbc) };
        println!("{:?}", info_str);
    }
*/

}
