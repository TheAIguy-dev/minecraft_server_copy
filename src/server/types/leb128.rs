const CONTINUATION_BIT: u8 = 0b10000000;

pub use varint::*;
pub mod varint {
    pub const MAX_BYTES: u8 = 5;

    use std::collections::VecDeque;

    use eyre::{eyre, ContextCompat, Result};
    use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

    use crate::server::util::ReadError;

    use super::CONTINUATION_BIT;

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
            let mut result: u32 = 0;

            for i in 0..MAX_BYTES {
                let byte: u8 = self.pop_front().context(ReadError::EndOfFile)?;

                // Check for extra bits
                if i == MAX_BYTES - 1 && byte & 0b01110000 != 0 {
                    return Err(eyre!(ReadError::Overflow));
                }

                result |= ((byte & !CONTINUATION_BIT) as u32) << (i * 7);

                if byte & CONTINUATION_BIT == 0 {
                    return Ok(result as i32);
                }
            }

            Err(eyre!(ReadError::Overflow))
        }
    }
    // impl ReadVarInt for &[u8] {
    //     fn read_varint(&mut self) -> Result<i32> {
    //         let mut result: i32 = 0;
    //         let mut shift: i32 = 0;
    //
    //         loop {
    //             ensure!(1 <= self.len(), Error::EOF);
    //
    //             let byte: u8 = self.get_u8();
    //             let low_bits: i32 = (byte & !CONTINUATION_BIT) as i32;
    //             result |= low_bits << shift;
    //
    //             if byte & CONTINUATION_BIT == 0 {
    //                 return Ok(result);
    //             }
    //
    //             shift += 7;
    //
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
            let mut result: u32 = 0;

            for i in 0..MAX_BYTES {
                let byte: u8 = self.read_u8().await?;

                // Check for extra bits
                if i == MAX_BYTES - 1 && byte & 0b01110000 != 0 {
                    return Err(eyre!(ReadError::Overflow));
                }

                result |= ((byte & !CONTINUATION_BIT) as u32) << (i * 7);

                if byte & CONTINUATION_BIT == 0 {
                    return Ok(result as i32);
                }
            }

            Err(eyre!(ReadError::Overflow))
        }
    }

    pub trait WriteVarInt {
        fn write_varint(&mut self, value: i32) -> usize;
    }
    impl WriteVarInt for Vec<u8> {
        fn write_varint(&mut self, value: i32) -> usize {
            self.reserve(MAX_BYTES as usize);
            let mut value: u32 = value as u32;
            let mut bytes_written: usize = 0;

            loop {
                let mut byte: u8 = (value & 0xFF) as u8;
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
                let mut byte: u8 = (value & 0xFF) as u8;
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

// TODO
pub use varlong::*;
pub mod varlong {
    pub const MAX_BYTES: i64 = 10;

    use eyre::{ensure, eyre, Result};

    use crate::server::util::ReadError;

    use super::CONTINUATION_BIT;

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
                ensure!(!self.is_empty(), ReadError::EndOfFile);

                let byte: u8 = self.remove(0);
                let low_bits: i64 = (byte & !CONTINUATION_BIT) as i64;
                result |= low_bits << shift;

                if byte & CONTINUATION_BIT == 0 {
                    return Ok(result);
                }

                shift += 7;

                if shift == MAX_BYTES * 7 {
                    return Err(eyre!(ReadError::Overflow));
                }
            }
        }
    }

    pub trait WriteVarLong {
        fn write_varlong(&mut self, value: i64) -> usize;
    }
    impl WriteVarLong for Vec<u8> {
        fn write_varlong(&mut self, value: i64) -> usize {
            self.reserve(MAX_BYTES as usize);
            let mut value: u64 = value as u64;
            let mut bytes_written: usize = 0;

            loop {
                let mut byte: u8 = (value & 0xFF) as u8;
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
