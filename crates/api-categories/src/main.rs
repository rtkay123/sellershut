mod state;

use axum::{response::Html, routing::get, Router};
use tracing::info;

#[tokio::main]
async fn main() {
    sellershut_services::telemetry::Handle::initialise("info");

    // build our application with a route
    let app = Router::new().route("/", get(handler));

    // run it
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}
