## Example

```rust
use mudbase::server::{ ServerHandler, Sessions, SessionsType };

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
    fn handle_message(message: &str, address: &str) {
      print!("{} says: {}", address, message);  
    }
}

fn main() {
    let server = Server::new();
    let listener = server.start("127.0.0.1", "7878");
    let sessions = server.sessions.clone();

    server.listen(listener, sessions);
}
```
