use actix_web::{HttpResponse, Responder};

pub async fn not_found() -> impl Responder {
    HttpResponse::NotFound().body("<h1>Error 404: Not Found</h1>")
}
