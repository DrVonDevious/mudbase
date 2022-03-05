## Mudbase Server
mudbase_server is a sub-module of the Mudbase library that makes it  
easier to start and configure a simple TCP server.

## Example

```rust
use mudbase::server::{
  ServerHandler,
  Sessions,
  SessionsType
  SessionType,
};

struct Server {
    sessions: SessionsType,
}

impl Server {
    fn new() -> Self {
        Self {
            sessions: Sessions::new(),
        }
    }
}

impl ServerHandler for Server {
    fn handle_message(session: &mut SessionType, message: &str, address: &str) {
      print!("{} says: {}", address, message);  

      // echos back the message
      Self::send(session, message);
    }
}

fn main() {
    let server = Server::new();
    let listener = server.start("127.0.0.1", "7878");
    let sessions = server.sessions.clone();

    server.listen(listener, sessions);
}
```
