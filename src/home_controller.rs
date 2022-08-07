use std::path::{Path, PathBuf};
use rocket::fs::NamedFile;
use crate::db_repository;

#[get("/")]
pub async fn index() -> Option<NamedFile> {
    println!("Connection: {}", "Ciao");
    NamedFile::open(Path::new("static/index.html")).await.ok()
}

#[get("/<file..>")]
pub async fn files(file: PathBuf) -> Option<NamedFile> {
    NamedFile::open(Path::new("static/").join(file)).await.ok()
}