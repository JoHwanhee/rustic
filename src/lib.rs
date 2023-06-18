// src/lib.rs
pub mod handlers {
    use std::io::{Read, Write};

    pub fn handle_connection<R: Read, W: Write, F>(mut reader: R, mut writer: W, generate_response: F)
        where F: Fn(&[u8]) -> Vec<u8>,
    {
        let mut buffer = [0; 1024];
        reader.read(&mut buffer).unwrap();

        let response = generate_response(&buffer);
        writer.write(&response).unwrap();
    }

    pub fn generate_response(_request: &[u8]) -> Vec<u8> {
        "HTTP/1.1 200 OK\r\n\r\nHello, World!".as_bytes().to_vec()
    }
}