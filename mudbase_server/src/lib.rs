pub mod prelude {
    use std::net::{TcpListener, TcpStream, SocketAddr};
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::thread;
    use std::sync::{Arc, Mutex};

    /// A type for a single client session.
    /// Contains a single TcpStream and an address
    #[allow(dead_code)]
    pub type SessionType = (TcpStream, SocketAddr);
    /// A type for a collection of client sessions
    /// Contains many sessions with SessionType
    #[allow(dead_code)]
    pub type SessionsType = Arc<Mutex<Vec<SessionType>>>;

    pub struct Sessions {}

    impl Sessions {
        /// Creates an empty Vector which can hold Sessions
        #[allow(dead_code)]
        pub fn new() -> SessionsType {
            Arc::new(Mutex::new(Vec::new()))
        }
    }

    #[allow(dead_code)]
    struct ClientState(String, SocketAddr);

    /// A basic server object that keeps track of all connected clients
    #[allow(dead_code)]
    pub struct MudServer {
        pub sessions: Sessions,
    }

    pub trait ServerHandler {
        /// Binds a TCP server to the given host address and port number,
        /// and returns a TcpListener
        ///
        /// # Arguments
        /// * `addr` - A string slice for the host address
        /// * `port` - A string slice for the port number
        fn start(&self, addr: &str, port: &str) -> TcpListener {
            TcpListener::bind(format!("{}:{}", addr, port)).unwrap()
        }

        /// Begins listening for client connections
        ///
        /// # Arguments
        /// * `listener` - A TcpListener, usually one that is returned by ServerHandler::start()
        /// * `sessions` - A collection of Sessions which connected clients can be added to
        fn listen(&self, listener: TcpListener, sessions: SessionsType) {
            let mut threads = vec![];

            loop {
                match listener.accept() {
                    Ok((stream, addr)) => {

                        let sessions = Arc::clone(&sessions);

                        threads.push(thread::spawn(move || {
                            Self::handle_client(stream, sessions, addr);
                        }));
                    }
                        Err(e) => println!("couldn't get client: {:?}", e),
                }
            }
        }

        fn handle_client(stream: TcpStream, sessions: Arc<Mutex<Vec<(TcpStream, SocketAddr)>>>, addr: SocketAddr) {

            let client_state = ClientState("".to_string(), addr);

            loop {
                let mut message = String::new();
                let mut reader = BufReader::new(stream.try_clone().unwrap());

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

                        sessions.remove(pos_to_remove);

                        return 
                    }
                }

                Self::handle_message(&message, &addr.to_string());
            }
        }

        /// Is called whenever a client sends data back to the server
        ///
        /// # Arguments
        /// * `message` - A string slice that contains the message sent by the client
        /// * `address` - The host address of the client that sent the message
        #[allow(unused_variables)]
        fn handle_message(message: &str, address: &str); 
    }
}
