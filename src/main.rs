use sqlx::postgres::PgPool;
use std::env;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")?;
    PgPool::connect(&database_url).await?;

    Ok(())
}
