use oxidisedb::run_server;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    time::{Duration, timeout},
};

async fn bind_ephemeral() -> std::io::Result<(TcpListener, std::net::SocketAddr)> {
    let listener = TcpListener::bind(("127.0.0.1", 0)).await?;
    let addr = listener.local_addr()?;
    Ok((listener, addr))
}

#[tokio::test]
async fn server_replies_once() -> std::io::Result<()> {
    // GIVEN
    let (listener, addr) = bind_ephemeral().await?;
    // Spawn the server and hand it over to another task
    tokio::spawn(run_server(listener));
    // create a client to connect to the server
    let mut client = TcpStream::connect(addr).await?;
    client.write_all(b"ping").await?;
    // The server replies with "reply: ping"
    let expected = b"reply: ping";
    let mut buf = vec![0u8; expected.len()];

    // WHEN
    // wait to seconds and read from client's buffer
    timeout(Duration::from_secs(2), client.read_exact(&mut buf))
        .await
        .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "read timed out"))??;

    // THEN
    assert_eq!(&buf, expected, "received expected reply");

    drop(client);
    Ok(())
}

#[tokio::test]
async fn server_handles_multiple_clients_concurrently() -> std::io::Result<()> {
    // create a server and give to an async task
    let (listener, addr) = bind_ephemeral().await?;
    tokio::spawn(run_server(listener));
    // create 2 clients
    let mut client_a = TcpStream::connect(addr).await?;
    let mut client_b = TcpStream::connect(addr).await?;
    // write to A and B
    client_a.write_all(b"from A").await?;
    client_b.write_all(b"from B").await?;
    // setup expected messages and buffer to read server reply into
    let expected_client_b_reply = b"reply: from B";
    let expected_client_a_reply = b"reply: from A";
    let mut reply_from_client_a = vec![0u8; expected_client_b_reply.len()];
    let mut reply_from_client_b = vec![0u8; expected_client_a_reply.len()];

    // WHEN
    // wait 1 second and read from both client A and B's buffers
    timeout(
        Duration::from_secs(1),
        client_b.read_exact(&mut reply_from_client_b),
    )
    .await
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "B read timed out"))??;

    timeout(
        Duration::from_secs(1),
        client_a.read_exact(&mut reply_from_client_a),
    )
    .await
    .map_err(|_| std::io::Error::new(std::io::ErrorKind::TimedOut, "A read timed out"))??;

    // THEN
    // assert that responses match
    assert_eq!(&reply_from_client_b, expected_client_b_reply);
    assert_eq!(&reply_from_client_a, expected_client_a_reply);

    Ok(())
}
