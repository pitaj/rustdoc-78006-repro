// #![feature(seek_convenience, read_initializer, can_vector, unsafe_block_in_unsafe_fn)]

// #![allow(dead_code, unused_variables)]

// pub use std::cmp;
// pub use std::fmt;
// pub use std::time;
// pub use std::convert;
// pub use std::ffi;
// pub use std::mem;
// pub use std::ptr;

// pub mod io {
//     pub use std::io::*;
//     pub const DEFAULT_BUF_SIZE: usize = 8 * 1024;
// }

mod bufreader;
mod tcp;
mod net {
    pub use crate::tcp::TcpStream;
    // pub use std::net::{Shutdown, SocketAddr, ToSocketAddrs};
}

fn main() {
    println!("Hello, world!");
}
