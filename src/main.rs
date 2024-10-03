mod utils;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;
use std::sync::Mutex;
use threadpool::ThreadPool;
use utils::queue::Queue;

fn listen_on(port: u16) -> TcpListener {
    TcpListener::bind(("127.0.0.1", port)).unwrap()
}

fn main() -> std::io::Result<()> {
    let listener = listen_on(8080);

    //queue
    let queue = Arc::new(Mutex::new(Queue::new()));

    // Create a thread pool with 4 worker threads
    let pool = ThreadPool::new(4);

    // Arc and Mutex safe concurrent
    let pool = Arc::new(Mutex::new(pool));

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let pool = Arc::clone(&pool);
                let queue = Arc::clone(&queue);
                // Safe multi_threaded execution using a thread pool
                pool.lock().unwrap().execute(move || {
                    let mut q = queue.lock().unwrap();
                    q.push(move || {
                        handle_client(stream); // Push the closure
                    });
                });
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
