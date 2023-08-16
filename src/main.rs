use dotenv;
use sqlx::sqlite::SqlitePool;
use std::env;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
}

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    dotenv::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;

    let users = get_users(&pool).await?;
    for user in users {
        println!("{:?}", user);
    }

    Ok(())
}

async fn get_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(User, "SELECT id, name, email FROM users")
        .fetch_all(pool)
        .await?;

    Ok(users)
}
