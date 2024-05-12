use std::mem::size_of;

use eyre::{ensure, Result};
use log::debug;
use paste::paste;

use crate::server::util::ReadError::*;

pub async fn test() -> Result<()> {
    let mut buf: PacketBuf = PacketBuf::new(vec![1, 2, 3], vec![]);
    debug!("{}", buf.read_u8()?);
    debug!("{}", buf.read_u8()?);
    debug!("{}", buf.read_u8()?);

    Ok(())
}

pub struct PacketBuf {
    pub data: Vec<u8>,
    pub data_pos: usize,
    pub metadata: Vec<u32>,
    pub metadata_pos: usize,
}
impl PacketBuf {
    pub fn new(data: Vec<u8>, metadata: Vec<u32>) -> Self {
        Self {
            data,
            data_pos: 0,
            metadata,
            metadata_pos: 0,
        }
    }
}

pub trait ReadBytes {
    read_bytes_def!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
}
impl ReadBytes for PacketBuf {
    read_bytes_impl!(u8 u16 u32 u64 u128 i8 i16 i32 i64 i128 f32 f64);
}

macro_rules! read_bytes_def {
    ($($ty:ty)+) => {
        $(
            paste! {
                fn [<peek_ $ty>](&mut self) -> Result<$ty>;
            }

            paste! {
                fn [<read_ $ty>](&mut self) -> Result<$ty>;
            }
        )+
    };
}
macro_rules! read_bytes_impl {
    ($($ty:ty)+) => {
        $(
            paste! {
                fn [<peek_ $ty>](&mut self) -> Result<$ty> {
                    ensure!(
                        self.data_pos + size_of::<$ty>() <= self.data.len(),
                        EndOfFile
                    );
                    Ok($ty::from_be_bytes(
                        self.data[self.data_pos..self.data_pos + size_of::<$ty>()].try_into()?,
                    ))
                }
            }

            paste! {
                fn [<read_ $ty>](&mut self) -> Result<$ty> {
                    self.[<peek_ $ty>]().and_then(|x| {
                        self.data_pos += size_of::<$ty>();
                        Ok(x)
                    })
                }
            }
        )+
    };
}
pub(crate) use {read_bytes_def, read_bytes_impl};
