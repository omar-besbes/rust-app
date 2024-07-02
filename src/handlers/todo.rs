use crate::dtos::todo::NewTodoDTO;
use crate::schemas::todo::Todo;
use crate::services::todo::{add_todo_service, list_todos_service};
use axum::body::Body;
use axum::response::Response;
use axum::{
    extract::Extension,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use sqlx::{PgPool, Pool, Postgres};
use tracing::{error, info};
use validator::Validate;

async fn add_todo(
    Extension(pool): Extension<PgPool>,
    Json(payload): Json<NewTodoDTO>,
) -> Result<Json<Todo>, Response<Body>> {
    info!("Received request to add todo: {:?}", payload);

    if let Err(validation_errors) = payload.validate() {
        error!("Validation error: {:?}", validation_errors);
        let error_message = match validation_errors.field_errors().get("title") {
            Some(errors) => {
                format!("{}", errors.first().unwrap())
            }
            None => {
                format!("Unknown validation error for field: title")
            }
        };

        return Err(Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .body(Body::from(error_message))
            .unwrap());
    }

    match add_todo_service(&pool, payload).await {
        Ok(created_todo) => {
            info!("Todo successfully added: {:?}", created_todo);
            Ok(Json(created_todo))
        }
        Err(e) => {
            error!("Failed to add todo: {}", e);
            Err(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::empty())
                .unwrap())
        }
    }
}

pub async fn list_todos(Extension(pool): Extension<PgPool>) -> Result<Json<Vec<Todo>>, StatusCode> {
    match list_todos_service(&pool).await {
        Ok(todos) => Ok(Json(todos)),
        Err(e) => {
            error!("Failed to fetch todos: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub fn router(pool: Pool<Postgres>) -> Router {
    Router::new()
        .route("/", post(add_todo))
        .route("/", get(list_todos))
        .layer(Extension(pool))
}
