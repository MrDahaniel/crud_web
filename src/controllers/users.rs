use actix_web::http::header::LOCATION;
use actix_web::{get, HttpRequest, HttpResponse, Responder};

#[get("/users")]
pub async fn user_list(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().body("Show users")
}

#[get("/users/{user_id}")]
pub async fn user_detail(req: HttpRequest) -> impl Responder {
    let response = match req.match_info().get("user_id") {
        Some(id_str) => match id_str.parse::<u32>() {
            Ok(id_int) => HttpResponse::Ok().body(format!("Details for user {}", id_int)),
            Err(_) => HttpResponse::NotFound().header(LOCATION, "/users").finish(),
        },
        None => HttpResponse::BadRequest()
            .header(LOCATION, "/users")
            .finish(),
    };

    println!("{:?}", response);

    return response;
}
