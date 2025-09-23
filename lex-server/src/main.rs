use fastrace::collector::Config as FastraceConfig;
use fastrace::collector::ConsoleReporter;
// use fastrace::prelude::*;

use lex_http::{HttpServer, HttpServerConfig};
use lex_json_repos::json_mode::JsonModeRepository;
use lex_service::service::Service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fastrace::set_reporter(ConsoleReporter, FastraceConfig::default());

    let mode_repo = JsonModeRepository::new("config/modes.json")?;
    let lex_service = Service::new(mode_repo);

    let server_config = HttpServerConfig { port: "3000" };
    let server = HttpServer::new(lex_service, server_config).await.unwrap();

    let res = server.run().await;

    fastrace::flush();
    res
}
