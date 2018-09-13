//! Bindings to the Linux Bluetooth low-complexity, subband codec (SBC) library

extern crate libsbc_sys as ffi;

use std::io::{self, Read};
use std::mem;

pub struct Decoder<R> {
    reader: R,
    sbc: Box<ffi::sbc_struct>,
}

impl<R> Decoder<R>
where
    R: Read, // maybe: seek + send
{

    //! Create a new decoder from the reader.
    pub fn new(reader: R) -> Decoder<R> {
        let mut sbc = unsafe { Box::new(mem::zeroed()) };
        unsafe { ffi::sbc_init(&mut *sbc, 0) };

        Decoder {
            reader,
            sbc,
        }
    }

    pub fn next_frame(&mut self) -> Result<(), ()> {
        Ok(())
    }


    pub fn info(&mut self) {
        let info_str = unsafe { ffi::sbc_get_implementation_info(&mut *self.sbc) };
        println!("{:?}", info_str);
    }

}
