pub mod download;
pub mod upload;

pub fn all_apis() -> actix_web::Scope {
    actix_web::web::scope("/api")
        .service(download::download)
        .service(upload::upload)
}
