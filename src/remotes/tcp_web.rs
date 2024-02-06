use serde::Deserialize;
use std::collections::VecDeque;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpStream;
#[derive(Debug, Deserialize)]
struct StructureAbstraction {
    name: String,
    children: Vec<StructureAbstraction>,
    url: Option<String>,
}
pub fn web_get_with_url(url: &str) -> Option<TcpStream> {
    println!("Connecting to {}...", url);
    let parts = url.split("/").collect::<Vec<&str>>();
    let host = parts[0];
    //println!("Recognized host: {}", host);
    let path = parts[1..].join("/");
    //println!("Recognized path: {}", path);
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

    Some(stream)
}
fn read_http_response(stream: &mut TcpStream) -> Result<String, std::io::Error> {
    let mut response: String = String::new();
    stream.read_to_string(&mut response)?;
    Ok(response)
}
pub fn receive_directory_structure(stream: &mut TcpStream, base_path: &str) -> bool {
    println!("Receiving directory structure...");
    let response = read_http_response(stream).unwrap();
    println!("Cleaning response...");
    let response_cleaned = clean_response(response);
    let structure = create_structure(response_cleaned);
    parse_and_write_structure(structure, base_path);
    true
}
fn clean_response(response: String) -> String {
    let full_lines = response.lines().collect::<Vec<&str>>();
    let mut clean_lines: Vec<&str> = Vec::new();
    for line_index in 0..full_lines.len() {
        if full_lines[line_index].trim().is_empty() {
            clean_lines = full_lines[line_index + 1..].to_vec();
            break;
        }
    }
    clean_lines.join("\n")
}
fn create_structure(from: String) -> Option<StructureAbstraction> {
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
    structure_process(&mut queue, here);
    true
}
fn structure_process(q: &mut VecDeque<StructureAbstraction>, path: &str) {
    if let Some(structure) = q.pop_front() {
        for child in structure.children {
            let child_path = format!("{}/{}", path, child.name);
            if let Some(url) = child.url {
                //this child's a file
                let stream = web_get_with_url(&url);
                if let Some(mut s) = stream {
                    let file_content = process_binary_file(&mut s);
                    if let Some(content) = file_content {
                        let mut file =
                            fs::File::create(&child_path).expect("Failed to create file.");
                        file.write_all(&content).expect("Failed to write to file.");
                    } else {
                        println!("Failed to clone file.");
                    }
                }
            } else {
                //this child's a directory
                fs::create_dir_all(&child_path).expect("Failed to create directory.");
                q.push_back(child);
                structure_process(q, &child_path);
            }
        }
    }
}
fn process_binary_file(stream: &mut TcpStream) -> Option<Vec<u8>> {
    let mut buffer: Vec<u8> = Vec::new();
    stream.read_to_end(&mut buffer).ok()?;
    try_clean_buffer(buffer)
}
fn try_clean_buffer(buffer: Vec<u8>) -> Option<Vec<u8>> {
    let binary: Vec<u8> =
        buffer.split(|&x| x == b"\n"[0]).collect::<Vec<&[u8]>>().join(&b"\n"[0]);
    if binary.len() < 10 {
        return None; //TODO
    }
    let clean_binary: Vec<u8> = binary[10..].to_vec();
    Some(binary)
}
