#![allow(unused_imports)]
use byteorder::WriteBytesExt;
use serde::Serialize;

use crate::server::types::{WriteVarInt, WriteVarLong};

#[tokio::test]
async fn test_varint() {
    let values: Vec<i32> = vec![
        0,
        1,
        2,
        127,
        128,
        255,
        25565,
        2097151,
        2147483647,
        -1,
        -2147483648,
    ];
    let results: Vec<Vec<u8>> = vec![
        vec![0],
        vec![1],
        vec![2],
        vec![127],
        vec![128, 1],
        vec![255, 1],
        vec![221, 199, 1],
        vec![255, 255, 127],
        vec![255, 255, 255, 255, 7],
        vec![255, 255, 255, 255, 15],
        vec![128, 128, 128, 128, 8],
    ];
    for i in 0..values.len() {
        let mut buf: Vec<u8> = vec![];
        buf.write_varint(values[i]).await;
        assert_eq!(buf, results[i], "test #{}", i);
    }
}

#[tokio::test]
async fn test_varlong() {
    let values: Vec<i64> = vec![
        0,
        1,
        2,
        127,
        128,
        255,
        2147483647,
        9223372036854775807,
        -1,
        -2147483648,
        -9223372036854775808,
    ];
    let results: Vec<Vec<u8>> = vec![
        vec![0],
        vec![1],
        vec![2],
        vec![127],
        vec![128, 1],
        vec![255, 1],
        vec![255, 255, 255, 255, 7],
        vec![255, 255, 255, 255, 255, 255, 255, 255, 127],
        vec![255, 255, 255, 255, 255, 255, 255, 255, 255, 1],
        vec![128, 128, 128, 128, 248, 255, 255, 255, 255, 1],
        vec![128, 128, 128, 128, 128, 128, 128, 128, 128, 1],
    ];
    for i in 0..values.len() {
        let mut buf: Vec<u8> = vec![];
        buf.write_varlong(values[i]).await;
        assert_eq!(buf, results[i], "test #{}", i);
    }
}
