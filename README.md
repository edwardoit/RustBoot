# Rust TCP Server with classic asynchronous core concept implemented with shared thread spawn into a deterministic thread pool

This project implements a basic TCP server in Rust using `TcpListener` and an asynchronous task queue infrastructure. The server listens for incoming connections and processes each connection asynchronously, responding with a simple message.

## Features

- **Asynchronous Response Handling**: The server uses async/await to handle multiple connections concurrently with shared thread.
- **Queue-based Infrastructure**: Incoming requests are queued and processed in an orderly fashion, making it easy to manage multiple clients.
- **TCP Listener**: Listens for connections on a specified port and serves responses.

## Prerequisites

To run this project, you need to have Rust installed. If you don’t have Rust installed, you can install it by running:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
