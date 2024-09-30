use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde_json::json;
use std::thread;

fn listen_on(port: u16) -> TcpListener {
    TcpListener::bind(("127.0.0.1", port)).unwrap()
}

fn main() -> std::io::Result<()> {
    let listener = listen_on(8080);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each client
                thread::spawn(move || handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection: {}", e);
            }
        }
    }
    Ok(())
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    // Read data from the stream
    stream.read(&mut buffer).expect("Failed to read from client!");
    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    // Create a JSON response
    let response_body = json!({ "body": "helloWorld" });
    let response_json = response_body.to_string();

    // Create the full HTTP response
    let response = format!(
        "HTTP/1.1 200 OK\r\n\
         Content-Type: application/json\r\n\
         Content-Length: {}\r\n\
         \r\n\
         {}",
        response_json.len(),
        response_json
    );

    // Write the response back to the client
    stream.write_all(response.as_bytes()).expect("Failed to write response!");
}
