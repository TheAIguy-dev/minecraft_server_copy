use thiserror::Error;

const CONTINUATION_BIT: u8 = 0b10000000;

#[derive(Error, Debug)]
pub enum Error {
    #[error("The number being read is larger than expected.")]
    Overflow,
    #[error("The end of the buffer was reached.")]
    EndOfFile,
}

pub use varint::*;
pub mod varint {
    pub const MAX_BYTES: i32 = 5;

    use std::collections::VecDeque;

    use eyre::{eyre, ContextCompat, Result};
    use log::debug;
    use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

    use super::{Error, CONTINUATION_BIT};

    pub struct VarInt(pub i32);
    impl VarInt {
        pub fn to_bytes(&self) -> Vec<u8> {
            let mut buf: Vec<u8> = vec![];
            buf.write_varint(self.0);
            buf
        }
    }

    pub trait ReadVarInt {
        fn read_varint(&mut self) -> Result<i32>;
    }
    impl ReadVarInt for VecDeque<u8> {
        fn read_varint(&mut self) -> Result<i32> {
            let mut result: i32 = 0;
            let mut shift: i32 = 0;

            loop {
                let byte: u8 = self.pop_front().context(Error::EndOfFile)?;
                let low_bits: i32 = (byte & !CONTINUATION_BIT) as i32;
                result |= low_bits << shift;

                if byte & CONTINUATION_BIT == 0 {
                    return Ok(result);
                }

                shift += 7;

                if shift == MAX_BYTES * 7 {
                    debug!("Error when reading varint");
                    return Err(eyre!(Error::Overflow));
                }
            }
        }
    }
    // impl ReadVarInt for &[u8] {
    //     fn read_varint(&mut self) -> Result<i32> {
    //         let mut result: i32 = 0;
    //         let mut shift: i32 = 0;

    //         loop {
    //             ensure!(1 <= self.len(), Error::EOF);

    //             let byte: u8 = self.get_u8();
    //             let low_bits: i32 = (byte & !CONTINUATION_BIT) as i32;
    //             result |= low_bits << shift;

    //             if byte & CONTINUATION_BIT == 0 {
    //                 return Ok(result);
    //             }

    //             shift += 7;

    //             if shift == MAX_BYTES * 7 {
    //                 return Err(eyre!(Error::Overflow));
    //             }
    //         }
    //     }
    // }

    pub trait AsyncReadVarInt {
        async fn async_read_varint(&mut self) -> Result<i32>;
    }
    impl<T: AsyncRead + Unpin> AsyncReadVarInt for T {
        async fn async_read_varint(&mut self) -> Result<i32> {
            let mut result: i32 = 0;
            let mut shift: i32 = 0;

            loop {
                let byte: u8 = self.read_u8().await?;
                let low_bits: i32 = (byte & !CONTINUATION_BIT) as i32;
                result |= low_bits << shift;

                if byte & CONTINUATION_BIT == 0 {
                    return Ok(result);
                }

                shift += 7;

                if shift == MAX_BYTES * 7 {
                    return Err(eyre!(Error::Overflow));
                }
            }
        }
    }

    pub trait WriteVarInt {
        fn write_varint(&mut self, value: i32) -> usize;
    }
    impl WriteVarInt for Vec<u8> {
        fn write_varint(&mut self, value: i32) -> usize {
            self.reserve(5);
            let mut value: u32 = value as u32;
            let mut bytes_written: usize = 0;

            loop {
                let mut byte: u8 = (value & 255) as u8;
                debug!("{}", value);
                value >>= 7;
                if value != 0 {
                    byte |= CONTINUATION_BIT;
                }

                self.push(byte);
                bytes_written += 1;

                if value == 0 {
                    return bytes_written;
                }
            }
        }
    }

    pub trait AsyncWriteVarInt {
        async fn async_write_varint(&mut self, value: i32) -> Result<usize>;
    }
    impl<T: AsyncWrite + Unpin> AsyncWriteVarInt for T {
        async fn async_write_varint(&mut self, value: i32) -> Result<usize> {
            let mut value: u32 = value as u32;
            let mut bytes_written: usize = 0;

            loop {
                let mut byte: u8 = (value & 255) as u8;
                value >>= 7;
                if value != 0 {
                    byte |= CONTINUATION_BIT;
                }

                self.write_u8(byte).await?;
                bytes_written += 1;

                if value == 0 {
                    return Ok(bytes_written);
                }
            }
        }
    }
}

pub use varlong::*;
pub mod varlong {
    pub const MAX_BYTES: i64 = 5;

    use eyre::{ensure, eyre, Result};

    use super::{Error, CONTINUATION_BIT};

    pub struct VarLong(pub i64);
    impl VarLong {
        pub fn to_bytes(&self) -> Vec<u8> {
            let mut buf: Vec<u8> = vec![];
            buf.write_varlong(self.0);
            buf
        }
    }

    pub trait ReadVarLong {
        fn read_varlong(&mut self) -> Result<i64>;
    }
    impl ReadVarLong for Vec<u8> {
        fn read_varlong(&mut self) -> Result<i64> {
            let mut result: i64 = 0;
            let mut shift: i64 = 0;

            loop {
                ensure!(!self.is_empty(), Error::EndOfFile);

                let byte: u8 = self.remove(0);
                let low_bits: i64 = (byte & !CONTINUATION_BIT) as i64;
                result |= low_bits << shift;

                if byte & CONTINUATION_BIT == 0 {
                    return Ok(result);
                }

                shift += 7;

                if shift == MAX_BYTES * 7 {
                    return Err(eyre!(Error::Overflow));
                }
            }
        }
    }

    pub trait WriteVarLong {
        fn write_varlong(&mut self, value: i64) -> usize;
    }
    impl WriteVarLong for Vec<u8> {
        fn write_varlong(&mut self, value: i64) -> usize {
            self.reserve(5);
            let mut value: u64 = value as u64;
            let mut bytes_written: usize = 0;

            loop {
                let mut byte: u8 = (value & 255) as u8;
                value >>= 7;
                if value != 0 {
                    byte |= CONTINUATION_BIT;
                }

                self.push(byte);
                bytes_written += 1;

                if value == 0 {
                    return bytes_written;
                }
            }
        }
    }
}
