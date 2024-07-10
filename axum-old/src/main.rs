use medstats::server::server;

#[tokio::main]
async fn main() {
    if let Err(e) = server().await {
        // Handle the error case here
        eprintln!("Server error: {}", e);
    }
}
