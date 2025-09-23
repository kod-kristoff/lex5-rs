use fastrace::collector::Config as FastraceConfig;
use fastrace::collector::ConsoleReporter;
// use fastrace::prelude::*;

use lex_http::{HttpServer, HttpServerConfig};
use lex_json_repos::json_mode::JsonModeRepository;
use lex_service::service::Service;

use crate::config::Config;

mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_env()?;

    fastrace::set_reporter(ConsoleReporter, FastraceConfig::default());

    let mode_repo = JsonModeRepository::new(&config.modes_path())?;
    let lex_service = Service::new(mode_repo);

    let server_config = HttpServerConfig {
        port: &config.server_port,
    };
    let server = HttpServer::new(lex_service, server_config).await.unwrap();

    let res = server.run().await;

    fastrace::flush();
    res
}
