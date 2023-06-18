use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;


fn handle_connection<R: Read, W: Write, F>(mut reader: R, mut writer: W, generate_response: F)
    where
        F: Fn(&[u8]) -> Vec<u8>,
{
    let mut buffer = [0; 1024];
    reader.read(&mut buffer).unwrap();

    let response = generate_response(&buffer);
    writer.write(&response).unwrap();
}

fn generate_response(_request: &[u8]) -> Vec<u8> {
    "HTTP/1.1 200 OK\r\n\r\nHello, World!".as_bytes().to_vec()
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        let reader = stream.try_clone().unwrap();

        handle_connection(reader, &mut stream, generate_response);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_generate_response() {
        let request = b"GET / HTTP/1.1\r\n\r\n";  // Example HTTP request
        let response = generate_response(request);

        let expected_response = "HTTP/1.1 200 OK\r\n\r\nHello, World!".as_bytes().to_vec();

        assert_eq!(response, expected_response);
    }

    #[test]
    fn test_handle_connection() {
        let request = b"GET / HTTP/1.1\r\n\r\n";
        let mut reader = Cursor::new(request);
        let mut writer = Cursor::new(vec![]);

        handle_connection(&mut reader, &mut writer, generate_response);

        let response = writer.into_inner();
        assert_eq!(response, b"HTTP/1.1 200 OK\r\n\r\nHello, World!");
    }
}