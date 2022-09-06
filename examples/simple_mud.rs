use mudbase::server::{ ServerHandler, Sessions, SessionsType, SessionType };

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
    fn on_connect(session: &mut SessionType) {
        println!("Client connected! {}", session.1);
        Self::send(session, "Welcome to MudBase!")
    }

    fn on_disconnect(session: &mut SessionType) {
        println!("Client disconnected! {}", session.1);
    }

    fn on_message(session: &mut SessionType, message: &str, address: &str) {
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

