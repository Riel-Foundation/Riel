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