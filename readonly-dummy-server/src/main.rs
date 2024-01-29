use actix_web::dev::Url;
use actix_web::HttpResponse;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use serde::Serialize;
use std::fs::{self, ReadDir};
use std::io::Read;
use std::path::{Path, PathBuf};
const URL_CONST: &str = "127.0.0.1:4000";
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(
                web::resource("/{username}/{repo}/exists")
                    .route(web::get().to(get_repo_confirmation)),
            )
            .service(web::resource("/{username}/{repo}/").route(web::get().to(get_repo_structure)))
            .service(actix_files::Files::new("/", "./repos"))
    })
    .bind(URL_CONST)?
    .run()
    .await
}
async fn get_repo_confirmation(info: web::Path<(String, String)>) -> impl Responder {
    let (username, repo) = info.into_inner();
    let path: PathBuf = Path::new("repos").join(username).join(repo);
    if path.exists() {
        HttpResponse::Ok().body("1")
    } else {
        HttpResponse::Ok().body("0")
    }
}
async fn get_repo_structure(info: web::Path<(String, String)>) -> impl Responder {
    let (username, repo) = info.into_inner();
    let path: PathBuf = Path::new("repos").join(username).join(repo);
    let response_object = get_dir_structure(path);
    let response = serde_json::to_string(&response_object).unwrap();
    HttpResponse::Ok().body(response)
}
#[derive(Serialize)]
struct dir_structure {
    name: String,
    children: Vec<dir_structure>,
    url: Option<String>,
}
fn get_dir_structure(path: PathBuf) -> dir_structure {
    let mut children: Vec<dir_structure> = Vec::new();
    let name: String;
    let url: Option<String>;
    if path.is_dir() {
        name = path.file_name().unwrap().to_str().unwrap().to_string();
        let dir: ReadDir = fs::read_dir(path).unwrap();
        for entry in dir {
            let entry: std::fs::DirEntry = entry.unwrap();
            let path: PathBuf = entry.path();
            children.push(get_dir_structure(path));
        }
        url = None;
    } else {
        name = path.file_name().unwrap().to_str().unwrap().to_string();
        let correct_path: &str = path.strip_prefix("repos").unwrap().to_str().unwrap();
        let url_path = format!("{}/{}", URL_CONST, correct_path);
        url = Some(url_path);
    }
    dir_structure {
        name,
        children,
        url,
    }
}
