use std::io;
use std::net::TcpListener;

use oxidisedb::serve_once;

fn main() -> io::Result<()> {
    let listener = TcpListener::bind(("127.0.0.1", 6349))?;
    println!("Listening on {}", listener.local_addr()?);

    loop {
        serve_once(&listener)?;
    }
}
