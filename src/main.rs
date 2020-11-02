#![feature(seek_convenience, read_initializer)]

pub use std::cmp;
pub use std::fmt;
pub use std::net;
pub mod io {
    pub use std::io::*;
    pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;
}

mod bufreader;

fn main() {
    println!("Hello, world!");
}
