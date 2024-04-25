use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
    thread,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    println!("server is started at port: 4221");

    for connection_stream in listener.incoming() {
        match connection_stream {
            Ok(stream) => {
                println!("accepted new connection");

                thread::spawn(|| {
                    handle_connection(stream);
                });
            }
            Err(err) => {
                eprintln!("error: {err}");
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let headers = parse_headers(&stream);
    println!("Received headers:\n{:?}", headers);

    let mut path = headers.get("path").unwrap().clone();
    if path.starts_with("/echo/") {
        let query = match path.strip_prefix("/echo/") {
            Some(value) => value,
            None => "",
        };
        let content_len = query.len();

        writeln!(stream, "HTTP/1.1 200 OK\r\nHost: localhost:4221\r\nContent-Type: text/plain\r\nContent-Length: {content_len}\r\n\r\n{query}\r\n\r\n").unwrap_or_else(|err| {
                        eprintln!("unable to write to stream: {err}");
                    });
    }

    if path.starts_with("/files/") {
        let mut process_args: Vec<_> = env::args().collect();
        process_args.remove(0);
        let dic = &process_args[1];
        let filename = path.clone().split_off(7);

        let full_path = format!("{dic}/{filename}");
        match File::open(full_path) {
            Ok(mut reader) => {
                let mut buffer = Vec::new();
                let file_len = reader.read_to_end(&mut buffer).unwrap();
                let http_header = format!("HTTP/1.1 200 OK\r\nHost: localhost:4221\r\nContent-Type: application/octet-stream\r\nContent-Length: {file_len}\r\n\r\n");
                stream.write(http_header.as_bytes()).unwrap();
                stream.write(&buffer).unwrap();
            }
            Err(_) => {
                writeln!(stream, "HTTP/1.1 404 Not Found\r\nHost: localhost:4221\r\nUser-Agent: curl/7.81.0\r\n\r\n").expect("unablet to write to stream");
            }
        }
    }

    path.remove(0);
    if path == "" {
        writeln!(stream, "HTTP/1.1 200 OK\r\nHost: localhost:4221\r\nContent-Type: text/plain\r\nContent-Length: 0\r\n\r\n{path}\r\n\r\n").unwrap_or_else(|err| {
                        eprintln!("unable to write to stream: {err}");
                    });
    } else {
        match headers.get(&path) {
            Some(value) => {
                let content_len = value.len();

                writeln!(stream, "HTTP/1.1 200 OK\r\nHost: localhost:4221\r\nContent-Type: text/plain\r\nContent-Length: {content_len}\r\n\r\n{value}\r\n\r\n").unwrap_or_else(|err| {
                             eprintln!("unable to write to stream: {err}");
                        });
            }
            None => {
                writeln!(stream, "HTTP/1.1 404 Not Found\r\nHost: localhost:4221\r\nUser-Agent: curl/7.81.0\r\n\r\n").expect("unablet to write to stream");
            }
        }
    }
}

fn parse_headers(mut stream: &TcpStream) -> HashMap<String, String> {
    let mut headers = String::new();
    let mut buf = [0; 1024];

    loop {
        stream.read(&mut buf).unwrap();
        let received = String::from_utf8_lossy(&buf);

        headers.push_str(&received);

        // Clear the buffer for the next read
        buf = [0; 1024];

        // Check if we've received the end of the headers
        if headers.contains("\r\n\r\n") {
            break;
        }
    }

    let mut rows = headers.split("\r\n").collect::<Vec<_>>();
    let mut map: HashMap<String, String> = HashMap::new();
    map.insert(
        "method".to_string(),
        rows[0].split(" ").collect::<Vec<_>>()[0].to_lowercase(),
    );
    map.insert(
        "path".to_string(),
        rows[0].split(" ").collect::<Vec<_>>()[1].to_string(),
    );

    rows.pop();
    rows.pop();
    rows.remove(0);

    for row in rows {
        let content = row.split(":").collect::<Vec<_>>();
        if content.len() == 2 {
            map.insert(content[0].to_lowercase(), content[1].trim().to_string());
        }
    }

    return map;
}
