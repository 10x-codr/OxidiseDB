use std::io;

use tracing::{error, info};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

pub async fn run_server(listener: TcpListener) -> io::Result<()> {
    loop {
        let (stream, addr) = listener.accept().await?;
        info!("Accepted {addr}");
        tokio::spawn(async move {
            if let Err(e) = handle_conn(stream).await {
                error!("connection {addr} error: {e}");
            }
        });
    }
}

// Handle one connection until the client closes it.
pub async fn handle_conn(mut stream: TcpStream) -> io::Result<()> {
    let mut buf = vec![0u8; 256];

    loop {
        let n = stream.read(&mut buf).await?; // was unwrap()
        if n == 0 {
            return Ok(()); // client closed
        }

        let msg = String::from_utf8_lossy(&buf[..n]);
        let response = format!("reply: {msg}");
        stream.write_all(response.as_bytes()).await?; // was unwrap()
        info!("Successfully sent response to client");
    }
}
