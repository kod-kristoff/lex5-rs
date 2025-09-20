use fastrace::collector::Config;
use fastrace::collector::ConsoleReporter;
// use fastrace::prelude::*;

use lex_http::{HttpServer, HttpServerConfig};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fastrace::set_reporter(ConsoleReporter, Config::default());

    let config = HttpServerConfig { port: "3000" };

    let server = HttpServer::new(config).await.unwrap();
    let res = server.run().await;

    fastrace::flush();
    res
}
