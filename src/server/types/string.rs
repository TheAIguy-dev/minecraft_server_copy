use std::{collections::VecDeque, string};

use eyre::{ensure, Context, Result};
use itertools::Itertools;

use crate::server::util::ReadError;

use super::{ReadVarInt, WriteVarInt};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct String(pub string::String);
impl String {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf: Vec<u8> = Vec::with_capacity(5 + self.0.len());
        buf.write_string(&self.0);
        buf
    }
}

pub trait ReadString {
    fn read_string(&mut self) -> Result<string::String>;
}
impl ReadString for VecDeque<u8> {
    fn read_string(&mut self) -> Result<string::String> {
        let len: usize = self.read_varint()? as usize;
        ensure!(len <= self.len(), ReadError::EndOfFile);
        let bytes: Vec<u8> = self.drain(..len).collect_vec();
        string::String::from_utf8(bytes).wrap_err("Invalid UTF-8")
    }
}

pub trait WriteString {
    fn write_string(&mut self, value: &str) -> usize;
}
impl WriteString for Vec<u8> {
    fn write_string(&mut self, value: &str) -> usize {
        let bytes_written: usize = self.write_varint(value.len() as i32);
        let bytes: &[u8] = value.as_bytes();
        self.extend_from_slice(bytes);
        bytes_written + bytes.len()
    }
}
