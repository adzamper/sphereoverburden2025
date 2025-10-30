use actix_cors::Cors;
use actix_web::{App, HttpServer, middleware};
mod electromagnetic;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("Starting server at http://localhost:8080");

    HttpServer::new(|| {
        let cors = Cors::permissive(); // Configure as needed for production

        App::new()
            .wrap(cors)
            .wrap(middleware::Logger::default())
            .service(routes::handle_calculation)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
