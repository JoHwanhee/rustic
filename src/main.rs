use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use rustic::handlers::{generate_response, handle_connection};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reader = stream.try_clone().unwrap();

        handle_connection(reader, &mut stream, generate_response);
    }
}

