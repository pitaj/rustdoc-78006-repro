```sh
$ cargo +stage2 doc
warning: unresolved link to `self::net::TcpStream::read`
  --> src/bufreader.rs:24:26
   |
24 | /// [`TcpStream::read`]: crate::net::TcpStream::read
   |                          ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the struct `TcpStream` has no field or associated item named `read`
   |
   = note: `#[warn(broken_intra_doc_links)]` on by default

warning: 1 warning emitted

    Finished dev [unoptimized + debuginfo] target(s) in 0.01s
```
