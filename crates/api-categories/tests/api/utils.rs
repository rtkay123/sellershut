use axum::{body::Body, http::Request, http::Response, Router};
use core_services::{
    cache::new_redis_pool_helper,
    state::{
        config::{env_var, Configuration, Environment},
        ServiceState,
    },
};
use sqlx::PgPool;
use tracing::trace;

use api_categories::{
    api::{ApiSchema, ApiSchemaBuilder},
    routes::router,
    state::ApiState,
};
use std::sync::Once;
use tower::util::ServiceExt;

static TRACING: Once = Once::new();

pub struct TestApp {
    pub router: Router,
    pub state: ApiState,
    pub schema: ApiSchema,
}

impl TestApp {
    pub async fn new(pool: PgPool, tx: tokio::sync::oneshot::Sender<u16>) -> Self {
        // Loads the .env file located in the environment's current directory or its parents in sequence.
        // .env used only for development, so we discard error in all other cases.
        dotenvy::dotenv().ok();

        // Set port to 0 so tests can spawn multiple servers on OS assigned ports.
        std::env::set_var("PORT", "0");

        // Setup tracing. Once.
        TRACING.call_once(|| {
            core_services::telemetry::TelemetryBuilder::new("error").build();
        });

        let nats_url = env_var("NATS_URL");

        let client = async_nats::connect(nats_url).await.unwrap();
        // Create a JetStream context.
        let jetstream_context = async_nats::jetstream::new(client);

        let state = ApiState {
            state: ServiceState {
                config: Configuration::new(),
                db_pool: pool,
                cache: new_redis_pool_helper().await,
                jetstream_context,
            },
        };

        trace!("building schema");
        let schema = ApiSchemaBuilder::build(state.clone());

        let router = router(schema.clone(), Environment::Development);

        tokio::spawn(api_categories::run(state.clone(), tx));

        Self {
            router,
            state,
            schema,
        }
    }

    pub async fn request(&self, req: Request<Body>) -> Response<Body> {
        self.router.clone().oneshot(req).await.unwrap()
    }
}
