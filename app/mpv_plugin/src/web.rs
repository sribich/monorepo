use axum::{Router, response::IntoResponse, routing::get};
use tokio::{sync::broadcast, task::JoinHandle};

pub struct WebHandler {
    shutdown_rx: broadcast::Receiver<()>,
}

impl WebHandler {
    pub fn new(shutdown_rx: broadcast::Receiver<()>) -> Self {
        Self { shutdown_rx }
    }

    pub fn run(self) -> JoinHandle<()> {
        let router = Router::<()>::new().route("/get_lines", get(get_lines));

        tokio::net::TcpListener::bind("127.0.0.1:9044");

        tokio::spawn(async {})
    }
}

async fn get_lines() -> impl IntoResponse {
    "test"
}
