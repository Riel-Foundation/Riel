use serde::{Deserialize, Serialize};
use serde_json::{json, Result as JsonResult, Value};
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::net::TcpStream;

#[derive(Debug, Deserialize)]
struct StructureAbstraction {
    name: String,
    children: Vec<StructureAbstraction>,
    url: Option<String>,
}
pub fn web_get_with_url(repo_url: &str) -> Option<TcpStream> {
    println!("Connecting to repository at {}...", repo_url);
    let parts = repo_url.split("/").collect::<Vec<&str>>();
    let host = parts[0];
    println!("Recognized host: {}", host);
    let path = parts[1..].join("/");
    println!("Recognized path: {}", path);
    let stream_result: Result<TcpStream, _> = TcpStream::connect(host);
    let mut stream: TcpStream = stream_result.ok()?;
    let request: String = format!(
        "GET /{} HTTP/1.1\r\n\
     Host: {}\r\n\
     Connection: close\r\n\r\n",
        path, host
    );
    stream
        .write_all(request.as_bytes())
        .expect("Failed to write to stream.");
    let mut buffer: String = String::new();
    Some(stream)
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
    println!("Receiving directory structure...");
    let response = read_http_response(stream).unwrap();
    println!("Cleaning response...");
    let response_cleaned = clean_response(response);
    println!("{}", response_cleaned);
    let structure = create_structure(response_cleaned, base_path);
    parse_and_write_structure(structure, base_path);
    true
}
fn clean_response(response: String) -> String {
    let mut full_lines = response.lines().collect::<Vec<&str>>();
    let mut clean_lines: Vec<&str> = Vec::new();
    //println!("Full lines: {:?}", full_lines);
    for line_index in 0..full_lines.len() {
        if full_lines[line_index].trim().is_empty() {
            //println!("Found empty line at index {}", line_index);
            clean_lines = full_lines[line_index + 1..].to_vec();
            break;
        }
    }
    //println!("Clean lines: {:?}", clean_lines);
    clean_lines.join("\n")
}
fn create_structure(from: String, here: &str) -> Option<StructureAbstraction> {
    let structure: StructureAbstraction = serde_json::from_str(&from).ok()?;

    Some(structure)
}
fn parse_and_write_structure(structure: Option<StructureAbstraction>, here: &str) -> bool {
    if structure.is_none() {
        return false;
    }
    let structure = structure.expect("This structure should not panic here.");
    let mut queue: VecDeque<StructureAbstraction> = VecDeque::new();
    queue.push_back(structure);
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        let mut path = here.to_string();
        path.push_str(&current.name);
        if current.children.is_empty() {
            println!("Creating file at {}", path);
            let mut file = File::create(path).unwrap();
            file.write_all(b"Hello, world!").unwrap();
        } else {
            println!("Creating directory at {}", path);
            create_dir_all(path).unwrap();
            for child in current.children {
                queue.push_back(child);
            }
        }
    }
    true
}