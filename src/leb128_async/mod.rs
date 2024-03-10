// TODO: Rewrite this fully (prob don't use async and writing tbh, using vecs should be faster; also should do this separately for i32/i64)

const CONTINUATION_BIT: u8 = 0x80;
const NOT_CONTINUATION_BIT: u8 = 0x7F;

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

pub mod write {
    use std::marker::Unpin;

    use tokio::io::{AsyncWrite, AsyncWriteExt};

    use super::Error;
    use super::CONTINUATION_BIT;

    pub async fn unsigned<W>(w: &mut W, mut val: u64) -> Result<usize, Error>
    where
        W: Unpin + AsyncWrite + ?Sized,
    {
        let mut bytes_written: usize = 0;
        let mut buf: Vec<u8> = Vec::with_capacity(10);

        loop {
            let mut byte: u8 = (val & 255) as u8;
            val >>= 7;
            if val != 0 {
                byte |= CONTINUATION_BIT;
            }

            buf.push(byte);
            bytes_written += 1;

            if val == 0 {
                w.write_all(&buf).await?;
                return Ok(bytes_written);
            }
        }
    }
}

pub mod read {
    use std::marker::Unpin;

    use tokio::io::{AsyncRead, AsyncReadExt};

    use super::Error;
    use super::{CONTINUATION_BIT, NOT_CONTINUATION_BIT};

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

            let low_bits: u64 = (buf[0] & NOT_CONTINUATION_BIT) as u64;
            result |= low_bits << shift;

            if buf[0] & CONTINUATION_BIT == 0 {
                return Ok(result);
            }

            shift += 7;
        }
    }
}
