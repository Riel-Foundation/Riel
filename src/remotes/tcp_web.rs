use std::collections::HashMap;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::net::TcpStream;
pub fn web_get_with_url(repo_url: &str) -> TcpStream {
    let mut stream: TcpStream = TcpStream::connect(repo_url).expect("Failed to connect to repository.");
    let request: String = format!(
        "GET / HTTP/1.1\r\n\
         Host: {}\r\n\
         Connection: close\r\n\r\n",
        repo_url
    );
    stream.write_all(request.as_bytes()).expect("Failed to write to stream.");
    let mut buffer: String = String::new();
    stream
}
pub fn receive_directory_structure(stream: &mut TcpStream, base_path: &str) -> bool {
    let mut buffer = [0; 1024];
    let mut paths_and_contents: HashMap<String, Vec<u8>> = HashMap::new();

    // Read the directory structure from the stream
    loop {
        let bytes_read = match stream.read(&mut buffer) {
            Ok(0) => break, // End of stream
            Ok(n) => n,
            Err(_) => return false, // Error reading from the stream
        };

        let data = buffer[..bytes_read].to_vec();
        let path = String::from_utf8_lossy(&data);
        paths_and_contents.insert(path.to_string(), data);
    }

    // Recreate the directory structure locally
    for (path, data) in paths_and_contents {
        let full_path = format!("{}/{}", base_path, path);

        if path.ends_with('/') {
            // It's a directory, create it
            if let Err(_) = create_dir_all(&full_path) {
                return false; // Error creating directory
            }
        } else {
            // It's a file, write its content to a file
            if let Err(_) = File::create(&full_path).and_then(|mut file| file.write_all(&data)) {
                return false; // Error creating or writing to file
            }
        }
    }

    true // Success
}
