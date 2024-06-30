mod health_check;

use axum::{body::Body, http::Request, http::Response, Router};
use sqlx::PgPool;
use std::sync::Once;
use tower::util::ServiceExt;

use crate::{
    api::{ApiSchema, ApiSchemaBuilder},
    routes::router,
    state::{config::Configuration, ApiState},
};

static TRACING: Once = Once::new();

pub struct TestApp {
    pub router: Router,
    pub state: ApiState,
    pub schema: ApiSchema,
}

impl TestApp {
    pub async fn new(pool: PgPool) -> Self {
        // Loads the .env file located in the environment's current directory or its parents in sequence.
        // .env used only for development, so we discard error in all other cases.
        dotenvy::dotenv().ok();

        // Set port to 0 so tests can spawn multiple servers on OS assigned ports.
        std::env::set_var("PORT", "0");

        // Setup tracing. Once.
        TRACING.call_once(|| {
            sellershut_services::telemetry::Handle::initialise();
        });

        let state = ApiState {
            config: Configuration::new(),
            db_pool: pool,
        };

        let schema = ApiSchemaBuilder::build(state.clone());

        tracing::debug!("Running migrations");

        let router = router(
            schema.clone(),
            crate::state::config::Environment::Development,
        );
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
