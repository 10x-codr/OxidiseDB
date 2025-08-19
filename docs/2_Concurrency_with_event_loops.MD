# Project Progress

## Dependencies Added
- **Tokio**: Provides the asynchronous runtime, TCP networking primitives, and async I/O traits. This is the foundation for handling multiple concurrent clients without blocking.
- **Tracing**: Introduced for structured, async-aware logging, replacing `println!` with `info!` and `error!`.
- **Tracing-subscriber**: Added to initialize the global subscriber and handle log formatting and output.

## Current Architecture
- The server binds to a TCP port (default `127.0.0.1:6379`).
- Incoming client connections are accepted in a loop.
- Each client connection is moved into its own asynchronous task using Tokio’s `spawn`.  
  This allows multiple clients to be handled concurrently.
- The connection handler reads incoming bytes and writes back a response prefixed with `"reply:"`.
- Logging is integrated at key points:
  - When a connection is accepted.
  - When a response is successfully written.
  - When an error occurs.

## Concurrency Model
The server uses an asynchronous, task-per-connection model:
- The main loop never blocks while a client is being served.
- While one connection awaits I/O, others continue making progress.
- This design scales naturally with Tokio’s runtime.

## Testing Strategy
- Wrote integration tests that spin up a server on an ephemeral port.
- Verified:
  - The server responds to a single message.
  - Multiple messages from the same client are handled.
  - Multiple clients can connect and be served concurrently.
- Tests use timeouts to avoid hanging indefinitely if something goes wrong.

## Known Limitations
- TCP is a raw byte stream; sending multiple messages quickly may result in them being read together and treated as a single response.
- No message framing or command parsing is in place yet.
- Server does not implement graceful shutdown — the loop runs indefinitely.

## Next Steps
- Introduce proper message framing (e.g., newline-delimited protocol or RESP).
- Add simple command parsing (`PING`, `ECHO`, etc.).
- Implement an in-memory key-value store for `SET`/`GET`.
- Add graceful shutdown mechanisms for cleaner test control.
- Enhance observability with structured logs and metrics.
