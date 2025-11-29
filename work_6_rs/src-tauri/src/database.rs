use once_cell::sync::OnceCell;
use sqlx::{Pool, Postgres};
use redis::{Client, RedisError, aio::MultiplexedConnection};
use my_models::{MyStudent, MyTableDDL};
use tokio::sync::Mutex;

static DB_POOL: OnceCell<Pool<Postgres>> = OnceCell::new();
static REDIS_CONN: OnceCell<Mutex<MultiplexedConnection>> = OnceCell::new();


pub async fn get_pg_pool() -> Result<&'static Pool<Postgres>, sqlx::Error> {
    let pool: &Pool<Postgres> = DB_POOL.get().ok_or(sqlx::Error::Protocol("DB pool not initialized".into()))?;
    Ok(pool)
}

pub async fn init_pg_pool() -> Result<(), sqlx::Error> {
    let url: &str = "postgres://postgres:postgres@localhost:5432/my_tauri_app_db";
    let pool: Pool<Postgres> = sqlx::postgres::PgPoolOptions::new()
        .max_connections(5)
        .connect(&url)
        .await?;
    DB_POOL
        .set(pool)
        .map_err(|_| sqlx::Error::Protocol("DB pool already initialized".into()))?; // положили в OnceCell
    Ok(())
}


pub async fn create_tables() -> Result<(), sqlx::Error> {
    // Все DDL в одном месте
    let ddls: [&str; 1] = [
        MyStudent::MY_CREATE_TABLE_DDL
    ];
    let pool: &Pool<Postgres> = DB_POOL.get()
        .ok_or(sqlx::Error::Protocol("DB pool not initialized".into()))?;
    // Начинаем транзакцию
    let mut tx: sqlx::Transaction<'_, Postgres> = pool.begin().await?;
    for ddl in ddls {
        sqlx::query(ddl).execute(&mut *tx).await?;
    }
    // Фиксируем транзакцию
    tx.commit().await?;
    Ok(())
}

// Redis функции с Mutex
pub async fn redis_get_students() -> Result<Option<Vec<MyStudent>>, String> {
    let mut guard = REDIS_CONN.get().ok_or("Redis not initialized")?.lock().await;
    MyStudent::get_from_redis( &mut *guard)
        .await
        .map_err(|e| e.to_string())
}

pub async fn redis_set_students(students: Vec<MyStudent>) -> Result<(), String> {
    let mut guard = REDIS_CONN.get().ok_or("Redis not initialized")?.lock().await;
    MyStudent::put_to_redis( students, &mut *guard)
        .await
        .map_err(|e| e.to_string())
}

pub async fn init_redis_connection() -> Result<(), RedisError> {
    let client: Client = Client::open("redis://localhost:6379")?;
    let conn: MultiplexedConnection = client.get_multiplexed_tokio_connection().await?;
    REDIS_CONN.set(Mutex::new(conn))
        .map_err(|_| RedisError::from((redis::ErrorKind::IoError, "Redis already initialized")))?;
    Ok(())
}
