use bip39::Mnemonic;
use itertools::Itertools;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tracing::info;
use tracing_subscriber;

async fn handle_connection(raw_stream: TcpStream, addr: SocketAddr) {
    println!("Incoming TCP connection from: {}", addr);

    let ws_stream = tokio_tungstenite::accept_async(raw_stream)
        .await
        .expect("Error during the websocket handshake occurred");
    println!("WebSocket connection established: {}", addr);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let words = 24;
    let mnemonic = Mnemonic::generate(words).unwrap();
    println!("{}", mnemonic.word_iter().join(" "));

    let addr = "0.0.0.0:4000".to_string();

    // Create the event loop and TCP listener we'll accept connections on.
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);

    // Let's spawn the handling of each connection in a separate task.
    // Let's spawn the handling of each connection in a separate task.
    while let Ok((stream, addr)) = listener.accept().await {
        tokio::spawn(handle_connection(stream, addr));
    }
}
