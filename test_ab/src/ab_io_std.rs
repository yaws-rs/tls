//! A/B NB I/O std (Read + Write)
//! The goal is to provide injection surface another side (i.e. server I/O against client I/O)
//!
//! # Tricks
//!
//! Trigger OpenSSL EWOULDBLOCK (mid-stream): std::io::Error::from_raw_os_error(11)

use core::marker::PhantomData;

#[derive(Debug)]
pub struct AbStreamForward {
    pub discard_write: usize,
    pub read_status: Result<usize, std::io::Error>,
}

/// std Read+Write mock
#[derive(Debug)]
pub struct AbStream<I, U> {
    //bytes_in: Vec<u8>,
    read_injector: I,
    bytes_out: Vec<u8>,

    user: U,
}

impl<I, U> AbStream<I, U>
where
    I: FnMut(&mut U, &mut [u8], &mut [u8]) -> AbStreamForward,
{
    pub fn with_read_injector(user: U, f: I) -> Self {
        Self {
            read_injector: f,
            bytes_out: Vec::with_capacity(16_384),
            user,
        }
    }
}

impl<I, U> std::io::Read for AbStream<I, U>
where
    I: FnMut(&mut U, &mut [u8], &mut [u8]) -> AbStreamForward,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        println!("stdRead triggered buf.len {}", buf.len());
        let fwd = (self.read_injector)(&mut self.user, buf, &mut self.bytes_out);

        if fwd.discard_write > 0 {
            let mut swap_vec = Vec::with_capacity(self.bytes_out.capacity());
            swap_vec.extend_from_slice(&self.bytes_out[fwd.discard_write..]);
            self.bytes_out = swap_vec;
        }

        fwd.read_status
    }
}

impl<I, U> std::io::Write for AbStream<I, U> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, std::io::Error> {
        println!("stdWrite triggered buf.len {}", buf.len());
        if buf.len() > 0 {
            self.bytes_out.extend_from_slice(buf);
            return Ok(buf.len());
        }
        Ok(0)
    }
    fn flush(&mut self) -> Result<(), std::io::Error> {
        Ok(())
    }
}
