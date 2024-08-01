use actix_web::HttpServer;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool = backend::configure_database().await?;

    println!("started");
    let _ = HttpServer::new(move || backend::create_app(pool.clone()))
        .bind(("127.0.0.1", 3000))?
        .run()
        .await;
    Ok(())
}
