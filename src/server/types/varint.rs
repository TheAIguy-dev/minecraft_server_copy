use crate::leb128_async;
use tokio::io::{AsyncRead, AsyncWrite};

pub struct VarInt(pub i32);
impl VarInt {
    pub async fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.write_varint(self.0).await;
        buf
    }
}

pub trait ReadVarInt {
    async fn read_varint(&mut self) -> i32;
}
impl<T: AsyncRead + Unpin> ReadVarInt for T {
    async fn read_varint(&mut self) -> i32 {
        leb128_async::read::unsigned(self).await.unwrap_or_default() as i32
    }
}

pub trait WriteVarInt {
    async fn write_varint(&mut self, value: i32) -> usize;
}
impl<T: AsyncWrite + Unpin> WriteVarInt for T {
    async fn write_varint(&mut self, value: i32) -> usize {
        leb128_async::write::unsigned(self, value as u32 as u64)
            .await
            .unwrap_or_default()
    }
}
