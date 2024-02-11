const CONTINUATION_BIT: u8 = 1 << 7;
fn low_bits_of_byte(byte: u8) -> u8 {
    byte & !CONTINUATION_BIT
}
fn low_bits_of_u64(val: u64) -> u8 {
    let byte: u64 = val & (std::u8::MAX as u64);
    low_bits_of_byte(byte as u8)
}

#[derive(Debug)]
pub enum Error {
    /// There was an underlying IO error.
    IoError(std::io::Error),
    /// The number being read is larger than can be represented.
    Overflow,
}
impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::IoError(e)
    }
}
impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        match *self {
            Error::IoError(ref e) => e.fmt(f),
            Error::Overflow => {
                write!(f, "The number being read is larger than can be represented")
            }
        }
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match *self {
            Error::IoError(ref e) => Some(e),
            Error::Overflow => None,
        }
    }
}

pub mod write {
    use super::Error;
    use super::{low_bits_of_u64, CONTINUATION_BIT};
    use std::marker::Unpin;
    use tokio::io::{AsyncWrite, AsyncWriteExt};

    pub async fn unsigned<W>(w: &mut W, mut val: u64) -> Result<usize, Error>
    where
        W: Unpin + AsyncWrite + ?Sized,
    {
        let mut bytes_written: usize = 0;
        loop {
            let mut byte: u8 = low_bits_of_u64(val);
            val >>= 7;
            if val != 0 {
                // More bytes to come, so set the continuation bit.
                byte |= CONTINUATION_BIT;
            }

            let buf: [u8; 1] = [byte];
            w.write_all(&buf).await?;
            bytes_written += 1;

            if val == 0 {
                return Ok(bytes_written);
            }
        }
    }
}

pub mod read {
    use super::Error;
    use super::{low_bits_of_byte, CONTINUATION_BIT};
    use std::marker::Unpin;
    use tokio::io::{AsyncRead, AsyncReadExt};

    pub async fn unsigned<R>(r: &mut R) -> Result<u64, Error>
    where
        R: Unpin + AsyncRead + ?Sized,
    {
        let mut result: u64 = 0;
        let mut shift: i32 = 0;

        loop {
            let mut buf: [u8; 1] = [0];
            r.read_exact(&mut buf).await?;

            if shift == 63 && buf[0] != 0x00 && buf[0] != 0x01 {
                while buf[0] & CONTINUATION_BIT != 0 {
                    r.read_exact(&mut buf).await?;
                }
                return Err(Error::Overflow);
            }

            let low_bits: u64 = low_bits_of_byte(buf[0]) as u64;
            result |= low_bits << shift;

            if buf[0] & CONTINUATION_BIT == 0 {
                return Ok(result);
            }

            shift += 7;
        }
    }
}
