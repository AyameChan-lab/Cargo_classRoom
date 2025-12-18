use std::sync::Arc;

use axum::{
    Router,
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
    routing::post,
};

use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    domain::value_objects::brawler_model::RegisterBrawlerModel,
    infrastructure::database::{
        postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawler_repository = Arc::new(BrawlerPostgres::new(db_pool));
    let brawler_usecase = Arc::new(BrawlersUseCase::new(brawler_repository));

    Router::new()
        .route("/register", post(register))
        .with_state(brawler_usecase)
}

async fn register(
    State(brawler_usecase): State<Arc<BrawlersUseCase<BrawlerPostgres>>>,
    Json(register_brawler_model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse {
    match brawler_usecase.register(register_brawler_model).await {
        Ok(_) => (StatusCode::CREATED, "Brawler registered successfully").into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
