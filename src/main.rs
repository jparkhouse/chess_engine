use chess_state::chess_pieces::{piece_structs::*, PieceEnum};
use env_logger::Builder;
use std::io::Write;

mod bitmask;
mod chess_state;
mod evaluation_engine;
mod shared;

fn main() {
    let mut builder = Builder::from_default_env();
    builder
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] - {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .init();
    log::info!("Initialised logger");

    println!("Hello, world!");
}
