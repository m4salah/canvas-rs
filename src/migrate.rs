use std::error::Error;

use canvas_rs::config;
use clap::Parser;
use sqlx::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // load environment variable from .env file
    dotenv::dotenv().ok();

    // load the env variable into config struct
    let config = config::Config::parse();

    // initialize the database pool
    let pool = PgPool::connect(&config.database_url).await?;

    // run the migration
    sqlx::migrate!("./migrations").run(&pool).await?;
    Ok(())
}
