use std::io;

use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber;

#[tokio::main]
async fn main() -> io::Result<()> {
    tracing_subscriber::fmt::init();
    let listener = TcpListener::bind(("127.0.0.1", 6379)).await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    info!("Listening for connections at: {:?}", local_addr);
    oxidisedb::run_server(listener).await
}
