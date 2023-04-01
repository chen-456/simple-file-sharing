mod api;
mod control;
mod file_ops;
mod listdir;
mod safe_path;
mod state;
mod tls;

use actix_web::{middleware::Logger, App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let app_state = actix_web::web::Data::new(state::AppState::new());
    // TODO: add GC for app_state
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .wrap(Logger::default())
            .service(api::all_apis())
            .route("/control", actix_web::web::get().to(control::websocket))
            .service(actix_files::Files::new("/", "public").index_file("index.html"))
    })
    .bind_rustls(
        ("localhost", 8080),
        tls::rustls_config().expect("create rustls configuration"),
    )?
    .run()
    .await
}
