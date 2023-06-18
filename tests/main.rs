#[cfg(test)]
mod tests {
    use std::io::Cursor;
    use rustic::handlers::{generate_response, handle_connection};

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