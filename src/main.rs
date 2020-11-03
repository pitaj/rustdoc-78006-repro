#![allow(unused_imports, dead_code)]

// Uncomment this and the warning will go away
// use crate::io::Read;

pub mod io {
    pub trait Read {
        fn read(&mut self, buf: &mut [u8]) -> usize;
    }
}

pub mod bufreader {
    use crate::io::Read;

    /// It can be excessively inefficient to work directly with a [`Read`] instance.
    /// For example, every [`Read::read`] call to [`TcpStream::read`] or [`TcpStream::yes`] on [`TcpStream`]
    ///
    /// [`TcpStream::yes`]: crate::net::TcpStream::yes
    /// [`TcpStream::read`]: crate::net::TcpStream::read
    /// [`Read::read`]: Read::read
    /// [`TcpStream`]: crate::net::TcpStream
    pub struct BufReader;
}

pub mod net {
    pub struct TcpStream;

    impl TcpStream {
        fn yes() -> bool {
            true
        }
    }

    impl crate::io::Read for TcpStream {
        fn read(&mut self, buf: &mut [u8]) -> usize {
            buf.len()
        }
    }
}

fn main() {
    println!("Hello, world!");
}
