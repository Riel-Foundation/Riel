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
            .service(web::resource("/{username}/{repo}/exists").route(web::get().to(get_repo_confirmation)))
            .service(web::resource("/{username}/{repo}/").route(web::get().to(get_repo_structure)))
    })
    .bind("127.0.0.1:4000")?
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
    HttpResponse::Ok().body("TODO")
}