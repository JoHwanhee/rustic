use std::io::prelude::*;
use std::net::TcpListener;

use rustic::handlers::{generate_response, handle_connection};

fn main() {
    TcpListener::bind("127.0.0.1:7878")
        .unwrap()
        .incoming()
        .for_each(|stream| {
            let mut stream = stream.unwrap();
            let reader = stream.try_clone().unwrap();

            handle_connection(reader, &mut stream, generate_response);
        })
}