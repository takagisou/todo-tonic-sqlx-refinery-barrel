use sqlx::mysql::MySqlPool;
use std::env;

pub async fn establish_connection() -> Result<MySqlPool, Box<dyn std::error::Error>> {
    let db_url = env::var("DATABASE_URL")?;
    print!("db_url: {}", db_url);
    Ok(MySqlPool::connect(&db_url).await?)
    //Ok(MySqlPool::builder().build(&db_url).await?)
}
