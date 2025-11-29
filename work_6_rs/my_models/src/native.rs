use redis::{AsyncTypedCommands, aio::MultiplexedConnection};
use sqlx::{Pool, Postgres, Row};
use std::{error::Error};

use crate::MyStudent;

impl crate::MyStudent {
    pub async fn put_to_redis(key_value: Vec<Self>, con: &mut MultiplexedConnection
        ) -> Result<(), Box<dyn Error>> {
        let json_str: String = serde_json::to_string(&key_value)?;
        con.set("my_key", json_str).await?;
        Ok(())
    }

    pub async fn get_from_redis(con: &mut MultiplexedConnection) -> Result<Option<Vec<Self>>, String> {
        let res: Option<String> = match con.get("my_key").await {
            Ok(res) => res,
            Err(e) => return Err(e.to_string())
        };
        match res {
            Some(json_str) => {
                let students: Vec<Self> = match serde_json::from_str(&json_str) {
                    Ok(t) => {
                        t
                    },
                    Err(e) => return Err(e.to_string())
                };
                Ok(Some(students))
            }
            None => Ok(None),
        }
    }

    pub async fn remove_from_redis(con: &mut MultiplexedConnection) -> Result<(), Box<dyn Error>> {
        con.del("my_key").await?;
        Ok(())
    }

    pub async fn save_to_db(&self, pool: &Pool<Postgres>) -> Result<(), String> {
        let query = "INSERT INTO t_students(id, first_name, middle_name, last_name) VALUES ($1, $2, $3, $4)";
        println!("{}", self.to_string());
        sqlx::query(query)
            .bind(self.id)
            .bind(self.first_name.clone())
            .bind(self.middle_name.clone())
            .bind(self.last_name.clone())
            .execute(pool)
            .await
            .map_err(|e| e.to_string())
            .map(|_| ())
    }

    pub async fn get_next_id(pool: &Pool<Postgres>) -> Result<i64, String> {
        let query: &str = "SELECT MAX(id) as max_id FROM t_students";
        let row = sqlx::query(query)
            .fetch_one(pool)
            .await
            .map_err(|e| format!("Database error: {}", e))?;
        let max_id: Option<i64> = row.get("max_id");
        Ok(max_id.unwrap_or(0) + 1)
    }

    pub async fn get_all_students(pool: &Pool<Postgres>) -> Result<Vec<MyStudent>, String> {
        let query = "SELECT * FROM t_students";
        sqlx::query_as::<_, MyStudent>(query)
            .fetch_all(pool)
            .await
            .map_err(|e| e.to_string())
    }
}
