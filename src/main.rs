mod utils;

use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};
use utils::req::request;

fn main() {
    let _ = request();

    let port = "3000";

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).expect("unable to bind port");

    for connection in listener.incoming() {
        match connection {
            Ok(stream) => {
                handle_connection(&stream);
            }
            Err(_) => println!("connection failded!!!"),
        }
    }
}

fn handle_connection(mut stream: &TcpStream) {
    let response = "HTTP/1.1 200 OK\r\n\r\nHello, World!";

    read_request(&stream);

    stream
        .write(response.as_bytes())
        .expect("failed to write response");

    stream.flush().expect("failed to flush output stream");
}

fn read_request(mut stream: &TcpStream) {
    let mut buffer = [0; 512];

    // Read request into buffer in chunks
    let mut total_bytes_read = 0;
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break, // No more data
            Ok(n) => n,
            Err(e) => {
                eprintln!("Error reading from stream: {}", e);
                break;
            }
        };

        total_bytes_read += bytes_read;

        if total_bytes_read > 1024 {
            eprintln!("Request too large, closing connection");
            return;
        }

        // Process the data as needed (you might want to decode it or analyze the request)

        if buffer[..bytes_read].contains(&b'\r') {
            break; // Assuming the end of the HTTP request is reached (this is just a simple example)
        }
    }

    println!(
        "Request: {:?}",
        String::from_utf8_lossy(&buffer[..total_bytes_read])
    );
}
