use actix_web::{get, HttpResponse, Responder};

#[get("/hello")]
async fn hello() -> impl Responder {
    HttpResponse::Ok()
        .content_type("application/json")
        .body(r#"{"msg": "Hello, world!"}"#)
}

pub fn all_apis() -> actix_web::Scope {
    actix_web::web::scope("/api").service(hello)
}
