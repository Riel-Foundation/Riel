use std::collections::{HashMap, VecDeque};
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::net::TcpStream;
pub fn web_get_with_url(repo_url: &str) -> TcpStream {
  println!("Connecting to repository at {}...", repo_url);
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
fn read_http_response(stream: &mut TcpStream) -> Result<String, std::io::Error> {
  let mut buffer = [0; 1024];
  let mut response = String::new();

  loop {
    let bytes_read = stream.read(&mut buffer)?;
    if bytes_read == 0 {
      break;
    }

    response.push_str(String::from_utf8_lossy(&buffer[..bytes_read]).as_ref());

    if response.contains("\r\n\r\n") {
      break;
    }
  }

  Ok(response)
}
pub fn receive_directory_structure(stream: &mut TcpStream, base_path: &str) -> bool {
  // Read the HTTP response headers
  if let Ok(http_response) = read_http_response(stream) {
    println!("HTTP Response:\n{}", http_response);
    // Check if the response starts with "HTTP/1.1 200 OK"
    if http_response.starts_with("HTTP/1.1 200 OK") {
      println!("Receiving directory structure...");
      let mut buffer = [0; 1024];
      let mut data_accumulator: Vec<u8> = Vec::new();
      let mut paths_and_contents: VecDeque<(String, Vec<u8>)> = VecDeque::new();

      // Read the directory structure from the stream
      loop {
        let bytes_read = match stream.read(&mut buffer) {
          Ok(0) => break, // End of stream
          Ok(n) => n,
          Err(_) => {
            println!("Error reading from the stream.");
            return false; // Error reading from the stream
          }
        };

        data_accumulator.extend_from_slice(&buffer[..bytes_read]);
      }

      println!("Directory structure copied.");

      // Deserialize paths and contents
      while let Some(separator_index) = data_accumulator.iter().position(|&b| b == 0x00) {
        let path = String::from_utf8_lossy(&data_accumulator[..separator_index]).to_string();
        data_accumulator.drain(..separator_index + 1); // Skip the separator

        if let Some(content_size) = data_accumulator.iter().position(|&b| b == 0xFF) {
          let content = data_accumulator.drain(..content_size).collect();

          paths_and_contents.push_back((path, content));
          data_accumulator.drain(..1); // Skip the content size indicator
        }
      }

      // Recreate the directory structure locally
      for (path, data) in paths_and_contents {
        let full_path = format!("{}/{}", base_path, path);
        println!("Creating {}...", full_path);

        if path.ends_with('/') {
          // It's a directory, create it
          if let Err(_) = create_dir_all(&full_path) {
            println!("Error creating directory.");
            return false; // Error creating directory
          }
        } else {
          // It's a file, write its content to a file
          if let Err(_) = File::create(&full_path).and_then(|mut file| file.write_all(&data)) {
            println!("Error creating or writing to file.");
            return false; // Error creating or writing to file
          }
        }
      }

      println!("Directory structure successfully received and recreated.");
      return true;
    }
  }

  false
}