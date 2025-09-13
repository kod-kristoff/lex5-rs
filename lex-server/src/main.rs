use lex_http::{HttpServer, HttpServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = HttpServerConfig { port: "3000" };

    let server = HttpServer::new(config).await.unwrap();
    server.run().await
}
