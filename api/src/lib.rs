use axum::{Router};
use std::env;


#[tokio::main]
async fn run_server() -> anyhow::Result<()> {

    dotenvy::dotenv().ok();
    let host = env::var("HOST").expect("Not Found HOST in .env file");
    let port = env::var("PORT").expect("Not Found PORT in .env file");

    let server_addr = format!("{host}:{port}");

    let app = Router::new();

    let sock = tokio::net::TcpListener::bind(server_addr).await.unwrap();
    axum::serve(sock, app).await?;

    Ok(())
}

pub fn main() {
    let result = run_server();

    if let Some(err) = result.err() {
        println!("Error = {err}");
    }
}
