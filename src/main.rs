use actix_web::HttpServer;
use dotenv::dotenv;

use auth_service::create_app::create_app;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "info");
    std::env::set_var("RUST_BACKTRACE", "1");
    dotenv().ok();
    env_logger::init();

    let server = HttpServer::new(move || create_app()).bind(("127.0.0.1", 15423))?;
    server.run().await
}
