use serde::{Deserialize, Serialize};
use serde_json::{json, Result as JsonResult, Value};
use std::collections::{HashMap, VecDeque};
use std::error::Error;
use std::ffi::{OsStr, OsString};
use std::fs;
use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::os::unix::ffi::OsStrExt;
use std::os::unix::fs::PermissionsExt;
use std::ptr::null;

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
    println!("{}", response_cleaned);
    let structure = create_structure(response_cleaned, base_path);
    parse_and_write_structure(structure, base_path);
    true
}
fn clean_response(response: String) -> String {
    let mut full_lines = response.lines().collect::<Vec<&str>>();
    let mut clean_lines: Vec<&str> = Vec::new();
    for line_index in 0..full_lines.len() {
        if full_lines[line_index].trim().is_empty() {
            clean_lines = full_lines[line_index + 1..].to_vec();
            break;
        }
    }
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
    structure_process(&mut queue, here);
    true
}
fn structure_process(mut q: &mut VecDeque<StructureAbstraction>, path: &str) {
    if let Some(structure) = q.pop_front() {
        for child in structure.children {
            let child_path = format!("{}/{}", path, child.name);
            if let Some(url) = child.url {
                //this child's a file
                let mut stream = web_get_with_url(&url);
                if let Some(mut s) = stream {
                    let file_content = process_file(&mut s, &url);
                    match file_content.failed {
                        true => {
                            println!("Failed to process file.");
                            continue;
                        }
                        false => match file_content.is_utf8 {
                            true => {
                                fs::write(&child_path, file_content.utf8)
                                    .expect("Failed to write file.");
                            }
                            false => {
                                fs::write(&child_path, file_content.binary)
                                    .expect("Failed to write file.");
                                // we now remove the header from the http
                                if try_clean_file(&child_path) {
                                    println!("Cleaned file. Binary file was cloned successfully.");
                                } else {
                                    println!("Failed to clean binary file.");
                                }
                            }
                        },
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
fn process_file(stream: &mut TcpStream, tcp_url: &str) -> FileContent {
    let mut buffer: String = String::new();
    let mut stream_clone: TcpStream = stream.try_clone().unwrap();
    let response: Result<usize, io::Error> = stream.read_to_string(&mut buffer);
    if response.is_err() && response.err().unwrap().kind() == std::io::ErrorKind::InvalidData {
        println!("Invalid data received. Non-utf8 files are not supported yet.");
        println!("Trying to read as binary...");
        return process_binary_file(&mut web_get_with_url(tcp_url).unwrap());
    }
    buffer = clean_response(buffer);
    FileContent {
        is_utf8: true,
        failed: false,
        utf8: buffer,
        binary: Vec::new(),
    }
}
fn process_binary_file(stream: &mut TcpStream) -> FileContent {
    let mut buffer: Vec<u8> = Vec::new();
    let response = stream.read_to_end(&mut buffer);
    if response.is_err() {
        println!("Failed to read binary file.");
        return FileContent {
            is_utf8: false,
            failed: true,
            utf8: String::new(),
            binary: Vec::new(),
        };
    }
    let binary: Vec<u8> = buffer;
    FileContent {
        is_utf8: false,
        failed: false,
        utf8: String::new(),
        binary,
    }
}
fn try_clean_file(path: &str) -> bool {
    let mut file = fs::File::open(path).expect("Failed to open file.");
    let mut buffer: Vec<u8> = Vec::new();
    let response = file.read_to_end(&mut buffer);
    if response.is_err() {
        println!("Failed to read binary file.");
        return false;
    }
    let binary: Vec<u8> = 
    buffer.split(|&x| x == b"\n"[0]).collect::<Vec<&[u8]>>()[9..].join(&b"\n"[0]);
    false
}
struct FileContent {
    is_utf8: bool,
    utf8: String,
    binary: Vec<u8>,
    failed: bool,
}
