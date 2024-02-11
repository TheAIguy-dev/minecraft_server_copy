use std::string;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use super::{ReadVarInt, WriteVarInt};

#[derive(Debug)]
pub struct String(pub string::String);
impl String {
    pub async fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = vec![];
        buf.write_var_int(self.0.len() as i32).await;
        buf.extend(self.0.as_bytes());
        buf
    }
}

pub trait ReadString {
    async fn read_string(&mut self) -> std::string::String;
}
impl<T: AsyncRead + Unpin> ReadString for T {
    async fn read_string(&mut self) -> string::String {
        let len: usize = self.read_var_int().await as usize;
        let mut bytes: Vec<u8> = vec![0; len];
        self.read_exact(&mut bytes).await.unwrap_or_default();
        string::String::from_utf8(bytes).unwrap_or_default()
    }
}

pub trait WriteString {
    async fn write_string(&mut self, value: &str) -> usize;
}
impl<T: AsyncWrite + Unpin> WriteString for T {
    async fn write_string(&mut self, value: &str) -> usize {
        self.write_var_int(value.len() as i32).await;
        self.write(value.as_bytes()).await.unwrap_or_default()
    }
}
