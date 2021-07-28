use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn handle_client(mut stream: TcpStream) {
     // Use a 50 byte buffer to hold incoming messages
    let mut data = [0 as u8; 50];
    while match stream.read(&mut data) {
        Ok(size) => {
            // Reading successfully, echo everything
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            //An error occurred, terminating connection
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

fn main() {
    //Listen to 127.0.0.1 port 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    // Accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 8080");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                    // connection succeeded, handle incoming messages
                    handle_client(stream)
                
            }
            Err(e) => {
                // connection failed
                println!("Error: {}", e);
            }
        }
    }
    // close the socket server
    drop(listener);
}