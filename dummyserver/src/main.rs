use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use std::fs::{self, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};
use actix_web::HttpResponse;
use serde::Serialize;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(web::resource("/{username}/{repo}").route(web::get().to(get_repo_info)))
            .service(web::resource("/{username}/{repo}/files").route(web::get().to(get_repo_files)))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}

#[derive(Serialize)]
struct FileResponse {
    file_name: String,
    content: String,
}

async fn get_repo_files(req: HttpRequest, info: web::Path<(String, String)>) -> impl Responder {
    let (username, repo) = info.into_inner();

    // Check if the repository exists
    if repository_exists(&username, &repo) {
        // Get a list of files in the repository
        let files = get_files_in_repository(&username, &repo);

        // Create a response with the content of each file
        let response: Vec<FileResponse> = files
            .iter()
            .map(|file| {
                let file_content = fs::read_to_string(file).unwrap_or_else(|_| String::from("Error reading file"));
                FileResponse {
                    file_name: file.file_name().unwrap().to_str().unwrap().to_string(),
                    content: file_content,
                }
            })
            .collect();

        HttpResponse::Ok().json(response)
    } else {
        HttpResponse::NotFound().json(format!("Repository '{}' under user '{}' does not exist.", repo, username))
    }
}

fn repository_exists(username: &str, repo: &str) -> bool {
    let repo_path = format!("./{}/{}", username, repo);
    Path::new(&repo_path).exists()
}
fn get_files_in_repository(username: &str, repo: &str) -> Vec<PathBuf> {
    let repo_path = format!("./{}/{}", username, repo);

    // Read the directory and collect file entries
    let entries: Vec<PathBuf> = fs::read_dir(repo_path)
        .unwrap()
        .filter_map(|entry| {
            entry.ok().map(|e| e.path())
        })
        .filter(|path| path.is_file())
        .collect();

    entries
}

async fn get_repo_info(info: web::Path<(String, String)>) -> impl Responder {
    let (username, repo) = info.into_inner();

    // Check if the repository exists (replace this with your actual logic)
    if repository_exists(&username, &repo) {
        format!("Repository '{}' under user '{}' exists.", repo, username)
    } else {
        format!("Repository '{}' under user '{}' does not exist.", repo, username)
    }
}
