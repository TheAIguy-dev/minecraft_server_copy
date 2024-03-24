#![forbid(unsafe_code)]
#![recursion_limit = "256"]

mod client;
mod config;
mod leb128_async;
mod server;
mod testing;

use std::env;
use std::io::Write;

use chrono::Local;
use env_logger::Builder;
use log::{debug, LevelFilter};

#[cfg(test)]
mod tests;

static PROTOCOL_VERSION: u16 = 763;
const SEED: i64 = 0;

#[tokio::main]
async fn main() {
    Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} {}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .filter(None, LevelFilter::Debug)
        .init();

    let args: Vec<String> = env::args().collect();
    debug!("{args:?}");

    match args.get(1).map(|s: &String| s.as_str()) {
        Some("" | "server") | None => {
            server::start().await;

            testing::test().await;
        }
        Some("client") => {
            client::start().await;
        }
        _ => panic!("Invalid arguments"),
    }
}
