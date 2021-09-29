mod controllers;
mod database;
mod models;

use actix_files as fs;
use actix_web::{web, App, HttpRequest, HttpServer, Responder};
use controllers::{error, landing, users};

async fn greet(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(fs::Files::new("/static", "./static"))
            .service(landing::landing)
            .service(users::user_list)
            .service(users::user_detail)
            .default_service(web::route().to(error::not_found))
        // .service(users::user_edit)
        // .service(users::user_delete)
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
