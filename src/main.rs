use actix_web::HttpServer;
use auth_service::create_app::create_app;
use dotenv::dotenv;

#[cfg(test)]
mod tests;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();
    let server = HttpServer::new(move || create_app()).bind(("127.0.0.1", 15423))?;
    server.run().await
}
