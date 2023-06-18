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


    #[test]
    fn test_generate_response_existing_file() {
        let request = b"GET /existing_file.txt HTTP/1.1\r\n\r\n";
        let response = generate_response(request);

        let expected_response = "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!";
        let expected_response_bytes = expected_response.as_bytes().to_vec();

        assert_eq!(response, expected_response_bytes);
    }

    #[test]
    fn test_generate_response_nonexistent_file() {
        let request = b"GET /nonexistent_file.txt HTTP/1.1\r\n\r\n";
        let response = generate_response(request);

        let expected_response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
        let expected_response_bytes = expected_response.as_bytes().to_vec();

        assert_eq!(response, expected_response_bytes);
    }
}