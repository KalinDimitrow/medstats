use actix_web::HttpServer;
use dotenv::dotenv;
use listenfd::ListenFd;
use log::info;
use std::env;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();
    env_logger::init();

    let pool = backend::configure_database().await?;

    let mut server = HttpServer::new(move || backend::create_app(pool.clone()));
    let mut listenfd = ListenFd::from_env();

    server = match listenfd.take_tcp_listener(0)? {
        Some(listener) => server.listen(listener)?,
        None => {
            let host = env::var("HOST").expect("Host not set");
            let port = env::var("PORT").expect("Port not set");
            server.bind(format!("{}:{}", host, port))?
        }
    };

    info!("Starting server");
    server.run().await?;
    Ok(())
}
