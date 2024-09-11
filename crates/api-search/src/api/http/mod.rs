use std::sync::Arc;

use axum::{
    extract::{Request, State},
    response::Html,
    routing::get,
    Router,
};
use opentelemetry::{global, trace::TraceContextExt};
use opentelemetry_http::HeaderExtractor;
use sentry::integrations::tower::{NewSentryLayer, SentryHttpLayer};
use tower_http::trace::TraceLayer;
use tracing::{info, info_span, Span};
use tracing_opentelemetry::OpenTelemetrySpanExt;

use crate::state::ApiState;

pub async fn serve(state: Arc<ApiState>) -> anyhow::Result<()> {
    let addr = state.state.config.listen_address;

    let app = Router::new()
        .route("/", get(handler))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        trace_id = tracing::field::Empty,
                    )
                })
                .on_request(on_request),
        )
        .layer(NewSentryLayer::new_from_top())
        .layer(SentryHttpLayer::with_transaction())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    info!(addr = ?addr, "listening");

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler(State(state): State<Arc<ApiState>>) -> Html<&'static str> {
    #[cfg(feature = "nlp")]
    {
        let texts = vec![
        "Who are you voting for in 2020?".into(),
        "The prime minister has announced a stimulus package which was widely criticized by the opposition.".into()
    ];
        let res = state.classifier.predict(texts).await;
        println!("{res:?}");
    }
    Html("<h1>Hello, World!</h1>")
}

fn on_request<B>(request: &Request<B>, span: &Span) {
    let parent_context = global::get_text_map_propagator(|propagator| {
        propagator.extract(&HeaderExtractor(request.headers()))
    });
    span.set_parent(parent_context);
    let trace_id = span.context().span().span_context().trace_id();
    span.record("trace_id", trace_id.to_string());
}
