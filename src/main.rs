mod utils;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
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
                //TODO possible implementation of shared thread
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

    let mut response_headers = utils::http::HttpHeaders::new();
    response_headers.add_header("Content-Type", "application/json");

    // Create the response body
    let response_body = utils::http::HttpBody::Json(r#"{"status": "success"}"#.to_string());

    // Create an HTTP response
    let response = utils::http::HttpResponse::new(
        utils::http::HttpVersion::HTTP20, // provide req analysis to res with same http version?
        200,
        "OK",
        response_headers,
        response_body,
    );

    //println
    println!("HTTP Response: {:#?}", response);

    // Write the response back to the client
    stream.write_all(response.as_string().as_bytes()).expect("Failed to write response!");
}
