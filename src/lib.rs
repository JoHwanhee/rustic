// src/lib.rs
pub mod handlers {
    use std::env;
    use std::fs::File;
    use std::io::{Read, Write};
    use std::path::PathBuf;

    pub fn handle_connection<R: Read, W: Write, F>(mut reader: R, mut writer: W, generate_response: F)
        where F: Fn(&[u8]) -> Vec<u8>,
    {
        let mut buffer = [0; 1024];
        reader.read(&mut buffer).unwrap();

        let response = generate_response(&buffer);
        writer.write(&response).unwrap();
    }


    pub fn generate_response(request: &[u8]) -> Vec<u8> {
        let binding = String::from_utf8_lossy(request);
        let filename = binding.lines().next().unwrap().split_whitespace().nth(1).unwrap();

        let current_dir = env::current_dir().unwrap();
        let file_path = current_dir.join(filename);

        match File::open(file_path) {
            Ok(mut file) => {
                let mut contents = Vec::new();
                file.read_to_end(&mut contents).unwrap();

                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
                    contents.len(),
                    String::from_utf8_lossy(&contents)
                );
                response.into()
            }
            Err(_) => {
                let response = "HTTP/1.1 404 NOT FOUND\r\nContent-Length: 0\r\n\r\n";
                response.into()
            }
        }
    }
}