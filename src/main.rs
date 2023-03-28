mod api;
mod control;

use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(api::all_apis())
            .route("/control", actix_web::web::get().to(control::websocket))
            .service(actix_files::Files::new("/", "public").index_file("index.html"))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
