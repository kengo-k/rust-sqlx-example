use dotenv;
use sqlx::sqlite::SqlitePool;
use sqlx::QueryBuilder;
use std::env;

#[derive(Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub address: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Debug)]
pub struct CreateUserRequest {
    pub name: String,
    pub email: String,
    pub address: Option<String>,
}

#[derive(Debug)]
pub struct UpdateUserRequest {
    pub name: Option<String>,
    pub email: Option<String>,
    pub address: Option<String>,
}

pub async fn create_pool() -> Result<SqlitePool, sqlx::Error> {
    dotenv::dotenv().expect("Failed to read .env file");
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = SqlitePool::connect(&database_url).await?;
    Ok(pool)
}

pub async fn truncate_table(pool: &SqlitePool, table_name: &str) -> Result<(), sqlx::Error> {
    let query = format!("delete from {}", table_name);
    sqlx::query(&query).execute(pool).await?;
    Ok(())
}

pub async fn get_users(pool: &SqlitePool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        "select id, name, email, address, created_at from users"
    )
    .fetch_all(pool)
    .await?;

    Ok(users)
}

pub async fn create_user(
    pool: &sqlx::SqlitePool,
    request: CreateUserRequest,
) -> Result<i64, sqlx::Error> {
    let result = sqlx::query!(
        "insert into users (name, email, address) values (?, ?, ?)",
        request.name,
        request.email,
        request.address
    )
    .execute(pool)
    .await?;

    Ok(result.last_insert_rowid())
}

pub async fn update_user(
    pool: &SqlitePool,
    id: i64,
    request: UpdateUserRequest,
) -> Result<u64, sqlx::Error> {
    let mut builder: QueryBuilder<sqlx::Sqlite> = QueryBuilder::new("update users set");

    if let Some(name) = &request.name {
        builder.push(" name = ");
        builder.push_bind(name);
    }
    if let Some(email) = &request.email {
        builder.push(" email = ");
        builder.push_bind(email);
    }
    if let Some(address) = &request.address {
        builder.push(" address = ");
        builder.push_bind(address);
    }

    builder.push(" where id = ");
    builder.push_bind(id);

    let query = builder.build();
    let result = query.execute(pool).await?;

    Ok(result.rows_affected())
}

pub async fn delete_user(pool: &SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
    let result = sqlx::query!("delete from users where id = ?", id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected())
}
