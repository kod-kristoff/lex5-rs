use lex_http::{HttpServer, HttpServerConfig};

#[tokio::main]
async fn main() -> Result<(), String> {
    let config = HttpServerConfig { port: "3000" };

    let server = HttpServer::new(config).await.unwrap();
    server.run().await
}
