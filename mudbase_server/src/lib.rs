pub mod prelude {
    use std::net::{TcpListener, TcpStream, SocketAddr};
    use std::io::prelude::*;
    use std::io::{ BufReader };
    use std::thread;
    use std::sync::{Arc, Mutex};

    /// A type for a single client session.
    /// Contains a single TcpStream and an address
    pub type SessionType = (TcpStream, SocketAddr);

    /// A type for a collection of client sessions
    /// Contains many sessions with SessionType
    pub type SessionsType = Arc<Mutex<Vec<SessionType>>>;

    pub struct Sessions {}

    impl Sessions {
        /// Creates an empty Vector which can hold Sessions
        #[allow(dead_code)]
        pub fn new() -> SessionsType {
            Arc::new(Mutex::new(Vec::new()))
        }
    }

    struct ClientState(String, SocketAddr);

    pub trait ServerHandler {
        /// Binds a TCP server to the given host address and port number,
        /// and returns a TcpListener
        ///
        /// ### Arguments
        /// * `addr` - A string slice for the host address
        /// * `port` - A string slice for the port number
        fn start(&self, addr: &str, port: &str) -> TcpListener {
            let listener: TcpListener = TcpListener::bind(format!("{}:{}", addr, port)).unwrap();
            println!("Server initialized!");
            return listener;
        }

        /// Begins listening for client connections
        ///
        /// ### Arguments
        /// * `listener` - A TcpListener, usually one that is returned by ServerHandler::start()
        /// * `sessions` - A collection of Sessions which connected clients can be added to
        fn listen(&self, listener: TcpListener, sessions: SessionsType) {
            let mut threads = vec![];

            println!("Server started. Listening at {}", listener.local_addr().unwrap());

            loop {
                match listener.accept() {
                    Ok((stream, addr)) => {

                        let sessions = Arc::clone(&sessions);

                        threads.push(thread::spawn(move || {
                            Self::_handle_client(stream, sessions, addr);
                        }));
                    }
                        Err(e) => println!("couldn't get client: {:?}", e),
                }
            }
        }

        fn _handle_client(stream: TcpStream, sessions: Arc<Mutex<Vec<(TcpStream, SocketAddr)>>>, addr: SocketAddr) {
            let client_state = ClientState("".to_string(), addr);
            let mut session: SessionType = (stream.try_clone().unwrap(), addr);
            let mut reader = BufReader::new(stream.try_clone().unwrap());

            Self::on_connect(&mut session);

            loop {
                let mut message = String::new();

                match reader.read_line(&mut message) {
                    Ok(_success) => (),
                    Err(_e) => {
                        let mut sessions = sessions.lock().unwrap();

                        let mut pos_to_remove = 0;

                        for (pos, session) in sessions.iter().enumerate() {
                            if session.1 == client_state.1 {
                                pos_to_remove = pos;
                            }
                        }

                        Self::on_disconnect(&mut session);

                        sessions.remove(pos_to_remove);

                        return 
                    }
                }

                Self::on_message(&mut session, &message, &addr.to_string());
            }
        }

        /// Sends a given message to the specified session
        ///
        /// ### Arguments
        /// * `session` - A SessionType to send data to
        /// * `message` - A string slice containing the message you want to send to the session
        fn send(session: &mut SessionType, message: &str) {
            session.0.write(message.as_bytes()).unwrap();
        }

        /// Sends a given message to all connected clients
        ///
        /// ### Arguments
        /// * `sessions` - A SessionsType containing all clients to send data to
        /// * `message` - A string slice containing the message you want to send to the clients
        fn send_all(sessions: &mut SessionsType, message: &str) {
            for session in sessions.lock().unwrap().iter_mut() {
                Self::send(session, message);
            }
        }

        /// Is called whenever a client connects to the server
        /// 
        /// ### Arguments
        /// * `session` - The SessionType of the connected client
        fn on_connect(session: &mut SessionType);

        /// Is called whenever a client disconnects from the server
        /// 
        /// ### Arguments
        /// * `session` - The SessionType of the disconnected client
        fn on_disconnect(session: &mut SessionType);

        /// Is called whenever a client sends data back to the server
        ///
        /// ### Arguments
        /// * `session` - The SessionType of the sender
        /// * `message` - A string slice that contains the message sent by the client
        /// * `address` - The host address of the client that sent the message
        fn on_message(session: &mut SessionType, message: &str, address: &str); 
    }
}
