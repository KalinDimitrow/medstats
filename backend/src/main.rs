use actix_web::{HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        backend::create_app()
    })
        .bind(("127.0.0.1", 3000))?
        .run()
        .await
}
