#[cfg(test)]
mod tests {

    use oxidisedb::{GREETING, serve_once};
    use std::io::{self, Read, Result};
    use std::net::{TcpListener, TcpStream};
    use std::sync::mpsc;
    use std::thread;

    #[test]
    fn server_sends_greeting_once() -> Result<()> {
        const PORT: u16 = 0;
        let listener = TcpListener::bind(("127.0.0.1", PORT))?;
        let addr: std::net::SocketAddr = listener.local_addr()?;

        // this will be used to send a signal when the listener is ready
        let (ready_tx, ready_rx) = mpsc::sync_channel::<()>(0);

        // move the listener to another thread and notify
        let server = thread::spawn(move || -> io::Result<()> {
            ready_tx.send(()).ok();
            serve_once(&listener)
        });

        // block until a signal from the server is received indicating that it has started
        ready_rx.recv().unwrap();

        // connect to the server
        let mut stream = TcpStream::connect(addr)?;
        let mut buf = Vec::new();
        // read message received in stream from server
        stream.read_to_end(&mut buf)?;

        assert_eq!(buf, GREETING);

        // make the server join the main thread
        server.join().unwrap()?;

        Ok(())
    }

    #[test]
    fn cannot_bind_same_port_twice() -> io::Result<()> {
        let first_listener = TcpListener::bind(("127.0.0.1", 0))?;
        let addr = first_listener.local_addr()?;

        let second_listener = TcpListener::bind(addr);
        assert!(
            second_listener.is_err(),
            "expected binding to fail on same port"
        );
        Ok(())
    }
}
