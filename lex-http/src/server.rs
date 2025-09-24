use std::sync::Arc;

use anyhow::Context;
use axum::{Router, routing};
use lex_service::ports::LexService;
use tokio::net;

use crate::handlers::{list_modes::list_modes, show_mode::show_mode};

/// Configuration for the HTTP server.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpServerConfig<'a> {
    pub port: &'a str,
}

#[derive(Debug, Clone)]
/// The global application state shared between all request handlers.
pub(crate) struct AppState<LS: LexService> {
    pub(crate) lex_service: Arc<LS>,
}

/// The application's HTTP server. The underlying HTTP package is opaque to module consumers.
pub struct HttpServer {
    router: axum::Router,
    listener: net::TcpListener,
}

impl HttpServer {
    /// Returns a new HTTP server bound to the port specified in `config`.
    pub async fn new(
        lex_service: impl LexService,
        config: HttpServerConfig<'_>,
    ) -> anyhow::Result<Self> {
        println!("Creating HttpServer!");
        // let trace_layer = tower_http::trace::TraceLayer::new_for_http().make_span_with(
        //     |request: &axum::extract::Request<_>| {
        //         let uri = request.uri().to_string();
        //         tracing::info_span!("http_request", method = ?request.method(), uri)
        //     },
        // );

        // Construct dependencies to inject into handlers.
        let state = AppState {
            lex_service: Arc::new(lex_service),
        };

        let router = api_routes()
            //     .layer(trace_layer)
            .with_state(state);

        let listener = net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
            .await
            .with_context(|| format!("failed to listen on {}", config.port))?;

        Ok(Self { router, listener })
    }

    pub fn local_addr(&self) -> std::io::Result<std::net::SocketAddr> {
        self.listener.local_addr()
    }

    /// Runs the HTTP server.
    pub async fn run(self) -> anyhow::Result<()> {
        println!("listening on {}", self.listener.local_addr().unwrap());
        // tracing::debug!("listening on {}", self.listener.local_addr().unwrap());
        axum::serve(self.listener, self.router)
            .await
            .context("received error from running server")?;
        Ok(())
    }
}

fn api_routes<LS: LexService>() -> Router<AppState<LS>> {
    Router::new()
        .route("/", routing::get(async || "Hello"))
        .route("/modes", routing::get(list_modes::<LS>))
        .route("/modeinfo/{mode}", routing::get(show_mode::<LS>))
}
