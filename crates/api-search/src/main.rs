mod state;

#[cfg(feature = "nlp")]
mod nlp;

use std::sync::Arc;

use anyhow::Result;
use state::ApiState;

use axum::{extract::State, response::Html, routing::get, Router};

#[tokio::main]
async fn main() -> Result<()> {
    let state = Arc::new(ApiState::initialise().await?);

    let app = Router::new().route("/", get(handler)).with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:1200")
        .await
        .unwrap();

    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();

    Ok(())
}

async fn handler(State(state): State<Arc<ApiState>>) -> Html<&'static str> {
    let texts = vec![
        "Who are you voting for in 2020?".into(),
        "The prime minister has announced a stimulus package which was widely criticized by the opposition.".into()
    ];
    let res = state.classifier.predict(texts).await;
    println!("{res:?}");
    Html("<h1>Hello, World!</h1>")
}
