use actix_files as fs;
use actix_web::{get, HttpRequest, Result};
use fs::NamedFile;

use std::path::PathBuf;

#[get("/")]
pub async fn landing(_req: HttpRequest) -> Result<fs::NamedFile> {
    let path: PathBuf = "static/html/result.html".parse().unwrap();
    Ok(NamedFile::open(path)?)
}
