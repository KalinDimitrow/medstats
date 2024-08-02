use actix_web::HttpServer;
use listenfd::ListenFd;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    let pool = backend::configure_database().await?;

    let mut server = HttpServer::new(move || backend::create_app(pool.clone()));
    let mut listenfd = ListenFd::from_env();

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => server.bind("localhost:3000")?,
    };
    server.run().await?;
    Ok(())
}
