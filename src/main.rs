use axum::Router;
use sqlx::postgres::PgPool;
use std::env;

mod dtos;
mod handlers;
mod schemas;
mod services;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // initialize tracing
    tracing_subscriber::fmt::init();

    let database_url = env::var("DATABASE_URL").expect("Missing DATABASE_URL");
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Could not connect to the database");

    let app = Router::new().nest("/todos", handlers::todo::router(pool));

    // run our app with hyper, listening globally on port 8080
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
