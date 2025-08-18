# 1. Basic TCP Listener

## Goal
At this initial stage of the project, the focus is to **create the simplest possible server**: one that listens on a TCP port, accepts incoming connections, and responds with a greeting message.  

We are deliberately keeping the scope narrow:
- No concurrency or threading yet.  
- No protocol parsing (e.g., RESP for Redis).  
- No persistence or command handling.  

The only responsibility of the server is to demonstrate that:  
1. It can successfully bind to a network port.  
2. It can accept a TCP connection from a client.  
3. It can send a predefined response back to the client.  

This ensures we’ve got a working foundation before adding complexity.

---

## Why Start Here?
Building a database like Redis from scratch can feel overwhelming. Redis handles networking, concurrency, serialization (RESP protocol), and persistence. To avoid being paralyzed by the full scope, it helps to **strip everything down to the first working milestone**:  

✅ A TCP listener proves the server can interact with the outside world.  
✅ A static greeting response proves data can flow from server → client.  
✅ A blocking accept loop provides a clear, easy-to-understand baseline before introducing concurrency.  

---

## What the Code Does
1. **Bind to a port** – The server binds to `127.0.0.1:6349`. This means the operating system will route incoming TCP packets on that port to our program.  
2. **Accept connections** – When a client connects (e.g., via `nc 127.0.0.1 6349`), the server accepts the connection.  
3. **Respond to the client** – Once connected, the server writes back a simple greeting message (`"Hello from server"`) before closing the connection.  

This “Hello World” for networking proves that the server is functional.

---

## What We Learned
- The **lifecycle of a TCP server**: bind → listen → accept → respond.  
- Rust’s `TcpListener` and `TcpStream` provide safe, ergonomic abstractions for sockets while still being close to the OS APIs.  
- Error handling with `Result` and the `?` operator makes failures explicit and manageable.  
- Even without concurrency, a simple loop of `listener.accept()` is enough to sequentially handle multiple client connections.  

---

## Next Steps
Once this baseline works, we’ll gradually add:  
- **Concurrency** – handling multiple clients at the same time.  
- **Protocol support** – understanding and responding to Redis-style commands (RESP).  
- **State management** – storing key/value pairs in memory.  
- **Persistence** – eventually writing data to disk.  

But first, having this minimal TCP listener ensures our foundation is solid.  

---

👉 This stage is the equivalent of *“Hello World” for systems programming*: proof that our Rust program can listen on a socket, talk to a client, and close the connection successfully.  
