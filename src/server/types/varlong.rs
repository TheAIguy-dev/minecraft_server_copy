use crate::leb128_async;
use tokio::io::{AsyncRead, AsyncWrite};

pub struct VarLong(pub i64);
impl VarLong {
    pub async fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.write_var_long(self.0).await;
        buf
    }
}

pub trait ReadVarLong {
    async fn read_var_long(&mut self) -> i64;
}
impl<T: AsyncRead + Unpin> ReadVarLong for T {
    async fn read_var_long(&mut self) -> i64 {
        leb128_async::read::unsigned(self).await.unwrap_or_default() as i64
    }
}

pub trait WriteVarLong {
    async fn write_var_long(&mut self, value: i64) -> usize;
}
impl<T: AsyncWrite + Unpin> WriteVarLong for T {
    async fn write_var_long(&mut self, value: i64) -> usize {
        leb128_async::write::unsigned(self, value as u64)
            .await
            .unwrap_or_default()
    }
}
