use std::net::{TcpListener, TcpStream};
use std::thread;
use std::io::Read;
use std::io::Write;

fn handle_client(mut stream: TcpStream) {
    loop {
        let mut read = [0; 1028];
        match stream.read(&mut read) {
            Ok(n) => {
                if n == 0 { 
                    println!("User has disconnected");
                    break;
                }
                stream.write(&read[0..n]).unwrap();
            }
            Err(err) => {
                panic!("{}", err);
            }
        }
    }
}

fn start(host: &str, port: &str) {
    println!("Starting server...");

    let listener = TcpListener::bind(format!("{}:{}", host, port)).unwrap();

    println!("Server listening at {}:{}", host, port);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("User has connected");

                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(_) => {
                println!("Error");
            }
        }
    }
}

fn main() {
    start("127.0.0.1", "7878");
}
