use actix_web::{web, App, HttpResponse, HttpServer};
use std::collections::HashMap;

fn get_dummy_directory_structure() -> HashMap<String, Vec<u8>> {
    let mut structure = HashMap::new();

    // Dummy directory with a file
    structure.insert("dummy_folder/".to_string(), Vec::new());
    structure.insert("dummy_folder/dummy_file.txt".to_string(), b"Hello, World!".to_vec());

    structure
}

async fn dummy_handler() -> HttpResponse {
    let dummy_structure = get_dummy_directory_structure();

    // Serialize the dummy structure to a binary format
    let mut serialized_data = Vec::new();
    for (path, content) in dummy_structure {
        // Use '\0' as a separator between paths and content
        serialized_data.extend_from_slice(path.as_bytes());
        serialized_data.push(0x00); // Null byte as separator
        serialized_data.extend_from_slice(&content);
        serialized_data.push(0xFF); // Indicator for content size
    }

    HttpResponse::Ok().body(serialized_data)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(web::resource("/").to(dummy_handler))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}
