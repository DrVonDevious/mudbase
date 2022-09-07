use mudbase::server::{ ServerHandler, Sessions, SessionsType, SessionType };
use serde::Deserialize;

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Exit {
    name: String,
    room: u32,
}

#[allow(dead_code)]
#[derive(Deserialize, Debug)]
struct Room {
    id: u32,
    name: String,
    description: String,
    exits: Vec<Exit>,
}

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

        Self::send(session, "Welcome to SimpleMUD!");

        let rooms_file = mudbase::parser::load_file("./examples/rooms.json");
        let rooms: Vec<Room> = serde_json::from_reader(rooms_file).expect("Error: failed to read json file");

        for room in rooms {
            println!("Loaded Room: {:?}", room);
        }
    }

    fn on_disconnect(session: &mut SessionType) {
        println!("Client disconnected! {}", session.1);
    }

    fn on_message(session: &mut SessionType, message: &str, address: &str) {
      print!("Got message from {}: {}", address, message);

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