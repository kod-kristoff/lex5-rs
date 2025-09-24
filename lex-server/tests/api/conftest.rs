use lex_http::{HttpServer, HttpServerConfig};
use lex_json_repos::json_mode::JsonModeRepository;
use lex_server::config::Config;
use lex_service::service::Service;

pub struct TestApp {
    pub address: String,
}

impl TestApp {
    pub async fn spawn_app() -> anyhow::Result<TestApp> {
        let config = Config::new("0".into(), "assets/testing".into());
        let mode_repo = JsonModeRepository::new(&config.modes_path())?;
        let lex_service = Service::new(mode_repo);

        let server_config = HttpServerConfig {
            port: &config.server_port,
        };
        let server = HttpServer::new(lex_service, server_config).await?;

        let port = server.local_addr()?.port();
        let address = format!("http://127.0.0.1:{}", port);

        tokio::spawn(async move { server.run().await });
        Ok(TestApp { address })
    }

    pub fn modes(&self) -> String {
        format!("{}/modes", self.address)
    }

    pub fn modeinfo(&self, mode: &str) -> String {
        format!("{}/modeinfo/{}", self.address, mode)
    }
}
