// src/lib.rs
use std::io::{self, Write};
use std::net::{TcpListener, TcpStream};

pub const GREETING: &[u8] = b"Hello from server";

pub fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    stream.write_all(GREETING)?;
    Ok(())
}

pub fn serve_once(listener: &TcpListener) -> io::Result<()> {
    let (stream, addr) = listener.accept()?;
    println!("Accepted connection from {addr}");
    handle_client(stream)
}
