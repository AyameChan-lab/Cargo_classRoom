use std::sync::Arc;

use axum::{
    Router,
    extract::{Extension, Json, State},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::post,
};

use crate::{
    application::use_cases::brawlers::BrawlersUseCase,
    domain::value_objects::{brawler_model::RegisterBrawlerModel, uploaded_image::UploadBase64Img},
    infrastructure::{
        database::{postgresql_connection::PgPoolSquad, repositories::brawlers::BrawlerPostgres},
        http::middleware::auth::authorization,
    },
};

pub fn routes(db_pool: Arc<PgPoolSquad>) -> Router {
    let brawler_repository = Arc::new(BrawlerPostgres::new(db_pool));
    let brawler_usecase = Arc::new(BrawlersUseCase::new(brawler_repository));

    Router::new()
        .route("/register", post(register))
        .route(
            "/avatar",
            post(upload_avatar).layer(middleware::from_fn(authorization)),
        )
        .with_state(brawler_usecase)
}

pub async fn upload_avatar(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<BrawlerPostgres>>>,
    Extension(user_id): Extension<i32>,
    Json(base64_image): Json<UploadBase64Img>,
) -> impl IntoResponse {
    match brawlers_use_case.upload_avatar(user_id, base64_image).await {
        Ok(uploaded_img) => (StatusCode::OK, Json(uploaded_img)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}

pub async fn register<T>(
    State(brawlers_use_case): State<Arc<BrawlersUseCase<T>>>,
    Json(register_brawler_model): Json<RegisterBrawlerModel>,
) -> impl IntoResponse
where
    T: crate::domain::repositories::brawlers::BrawlerRepository + Send + Sync,
{
    match brawlers_use_case.register(register_brawler_model).await {
        Ok(passport) => (StatusCode::CREATED, Json(passport)).into_response(),
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response(),
    }
}
