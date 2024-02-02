use native_tls::TlsConnector;
use std::{
    io::{Read, Write, Error},
    net::TcpStream,
};

pub fn request() -> Result<String, Error> {
    let addr = "jsonplaceholder.typicode.com";
    let path = "/todos/1";

    let stream = TcpStream::connect(format!("{}:443", addr))?;
    let tls_connector = TlsConnector::new().unwrap();
    let mut tls_stream = tls_connector.connect(addr, stream).unwrap();

    let mut response = String::new();

    // Define the HTTP request
    let request = format!(
        "GET {} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
        path, addr
    );

    if let Err(e) = tls_stream.write(request.as_bytes()) {
        eprintln!("error writing request: \r\n{}", e);
    }

    if let Err(e) = tls_stream.read_to_string(&mut response) {
        eprintln!("error reading request: \r\n{}", e);
    }

    println!("response: \r\n\n{}", response);

    return Ok(response);
}
