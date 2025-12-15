use std::collections::HashSet;
use redis::{AsyncTypedCommands, ToRedisArgs, ToSingleRedisArg, aio::MultiplexedConnection, streams::StreamReadReply};
use crate::{Friend, MyAction, MyActionKind, MyFileParsed, MyNotification, MyProduct, MyProductInCart, MyTask, MyTaskPriority};

// Task 1

impl ToRedisArgs for MyFileParsed {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite {
            let json: String = serde_json::to_string(self).unwrap();
            out.write_arg(json.as_bytes());
    }
}

// Task 2
impl ToRedisArgs for MyAction {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite {
           let json: String = serde_json::to_string(self).unwrap();
            out.write_arg(json.as_bytes()); 
    }
}

impl ToRedisArgs for MyActionKind {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite {
        let json: String = serde_json::to_string(self).unwrap();
        out.write_arg(json.as_bytes());
    }
}

impl ToSingleRedisArg for MyAction {}


// Task 3 

pub struct FriendService(pub String);

impl FriendService {

    pub fn new(arg: String) -> Self {
        Self(arg)
    }

    pub async fn add_friend(&self, user_id: u64, friend_id: u64, conn: &mut MultiplexedConnection) -> Result<(), String> {
        // Добавляем в обоих направлениях (симметричные отношения)
        let user_key: String = format!("friends:{}", user_id);
        let friend_key: String = format!("friends:{}", friend_id);
        let _: (i32, i32, i32, i32) = redis::pipe().atomic()
            .sadd(&user_key, friend_id)
            .sadd(&friend_key, user_id)
            // Удаляем из входящих заявок, если они были
            .srem(format!("incoming_requests:{}", user_id), friend_id)
            .srem(format!("incoming_requests:{}", friend_id), user_id)
            .query_async(conn).await.map_err(|e: redis::RedisError| e.to_string())?;
        Ok(())
    }

    pub async fn remove_friend(&self, user_id: u64, friend_id: u64, conn: &mut MultiplexedConnection) -> Result<(bool, bool), String> {
        let user_key: String = format!("friends:{}", user_id);
        let friend_key: String = format!("friends:{}", friend_id);
        let (user_removed, friend_removed): (bool, bool) = redis::pipe().atomic()
            .srem(&user_key, friend_id)
            .srem(&friend_key, user_id)
            .query_async(conn)
            .await.map_err(|e: redis::RedisError| e.to_string())?;
        Ok((user_removed, friend_removed))
    }

    // 1.1. Получение списка друзей
    pub async fn get_friends(&self, user_id: u64, conn: &mut MultiplexedConnection) -> Result<Vec<u64>, String> {
        let key: String = format!("friends:{}", user_id);
        let friends: std::collections::HashSet<String> = conn.smembers(&key).await.map_err(|e: redis::RedisError| e.to_string())?;
        let friends_result: Result<Vec<u64>, _> = friends.into_iter().map(|s: String| s.parse::<u64>()).collect();
        friends_result.map_err(|e: std::num::ParseIntError| e.to_string())
    }
    
    // 1.1. Проверка, являются ли пользователи друзьями
    pub async fn is_friend(&self, user_id: u64, friend_id: u64, conn: &mut MultiplexedConnection) -> Result<bool, String> {
        let key: String = format!("friends:{}", user_id);
        let is_member: bool = conn.sismember(&key, friend_id).await.map_err(|e| e.to_string())?;
        Ok(is_member)
    }

    // 1.3. Рекомендации друзей (друзья друзей)
    pub async fn get_friend_recommendations(&self, user_id: u64, limit: Option<usize>, conn: &mut MultiplexedConnection) -> Result<Vec<u64>, String> {
        let user_key: String = format!("friends:{}", user_id);
        // Получаем всех друзей пользователя как строки
        let friends_set: std::collections::HashSet<String> = conn.smembers(&user_key).await.map_err(|e: redis::RedisError| e.to_string())?;
        if friends_set.is_empty() {
            return Ok(Vec::new());
        }
        // Преобразуем строки в числа и создаем ключи
        let mut friend_ids: Vec<u64> = Vec::new();
        let mut friend_keys: Vec<String> = Vec::new();
        for friend_str in friends_set {
            match friend_str.parse::<u64>() {
                Ok(friend_id) => {
                    friend_ids.push(friend_id);
                    friend_keys.push(format!("friends:{}", friend_id));
                }
                Err(e) => return Err(format!("Invalid friend ID '{}': {}", friend_str, e)),
            }
        }
        // Используем SUNION для объединения всех множеств друзей
        let all_friends_of_friends_set: HashSet<String> = if friend_keys.len() == 1 {
            conn.smembers(&friend_keys[0])
                .await
                .map_err(|e: redis::RedisError| e.to_string())?
        } else {
            conn.sunion(friend_keys)
                .await
                .map_err(|e: redis::RedisError| e.to_string())?
        };
        // Преобразуем в Vec<u64> и фильтруем
        let mut all_friends_of_friends: Vec<u64> = Vec::new();
        for friend_str in all_friends_of_friends_set {
            if let Ok(friend_id) = friend_str.parse::<u64>() {
                if friend_id != user_id && !friend_ids.contains(&friend_id) {
                    all_friends_of_friends.push(friend_id);
                }
            }
        }
        // Применяем лимит
        if let Some(limit) = limit {
            all_friends_of_friends.truncate(limit);
        }
        Ok(all_friends_of_friends)
    }

    pub async fn get_profile_by_id(user_id: u64, conn: &mut MultiplexedConnection) -> Result<Friend, String> {
        let profile_key: String = format!("user:{}:profile", user_id);
        let Some(username) = conn.hget(&profile_key, "username").await.map_err(|e: redis::RedisError| e.to_string())? else {
            return Err("username in redis empty !".to_string());
        };
        Ok(Friend { id: user_id, username})
    }

}

// Task 4
pub struct MyTaskService {}

impl ToRedisArgs for MyTask {
    fn write_redis_args<W>(&self, out: &mut W)
    where
        W: ?Sized + redis::RedisWrite {
        let json = serde_json::to_string(self).unwrap();
        out.write_arg(json.as_bytes());
    }
}

impl ToSingleRedisArg for MyTask {}

impl From<MyTaskPriority> for u8 {
    fn from(value: MyTaskPriority) -> Self {
        match value {
            MyTaskPriority::Common => 0,
            MyTaskPriority::Emergancy => 1,
            MyTaskPriority::Expired => 2,
        }
    }
}

impl MyTaskService {
    pub async fn add_task(task: MyTask, conn: &mut MultiplexedConnection) -> Result<(), String> {
        conn.zadd("my_tasks", task.str.clone(), u8::from(task.priority)).await.map_err(|e: redis::RedisError| e.to_string())?;
        Ok(())
    }

    pub async fn remove_task(task: MyTask, conn: &mut MultiplexedConnection) -> Result<(), String> {
        conn.zrem("my_tasks", task.str.clone()).await.map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_sorted_tasks_by_priority(conn: &mut MultiplexedConnection) -> Result<Vec<MyTask>, String> {
        let res: Vec<String> = {conn.zrange("my_tasks", 0, -1).await.map_err(|e: redis::RedisError| e.to_string())? };
        let mut result: Vec<MyTask> = vec![];
        for i in res {
            println!("{}", i.clone());
            let Some(priority) = conn.zscore("my_tasks", i.clone()).await.map_err(|e| e.to_string())? else { return Ok(result); };
            result.push(MyTask { str: i.clone(), priority: From::from(priority) });
        }
        Ok(result)
    }
}

// Task 5 
pub struct MyProductCartServices {}

impl MyProductCartServices {
    pub async fn add_product(product: MyProduct, count: u64, conn: &mut MultiplexedConnection) -> Result<(), String> {

        println!("=== ADD PRODUCT DEBUG ===");
        println!("product.id: {:?}", product.id);
        println!("product.name: {:?}", product.name);
        println!("product.name.is_empty(): {}", product.name.is_empty());
        println!("product.cost: {:?}", product.cost);
        println!("count: {:?}", count);
         // СОХРАНЯЕМ строки в переменные!
        let args: Vec<(String, String)> = vec![
            (format!("prod:{}", product.id), product.name),
            (format!("prod:{}-count", product.id), count.to_string()),
            (format!("prod:{}-cost", product.id), product.cost.to_string()),
        ];
        conn.hset_multiple("my_cart", &args).await.map_err(|e: redis::RedisError| e.to_string())?;
        Ok(())
    }

    pub async fn remove_product(product_id: u64, conn: &mut MultiplexedConnection) -> Result<(), String> {
        conn.hdel("my_cart", &format!("prod:{}", product_id)).await.map_err(|e: redis::RedisError| e.to_string())?;
        conn.hdel("my_cart", &format!("prod:{}-count", product_id)).await.map_err(|e: redis::RedisError| e.to_string())?;
        conn.hdel("my_cart", &format!("prod:{}-cost", product_id)).await.map_err(|e: redis::RedisError| e.to_string())?;
        Ok(())
    } 

    pub async fn get_all_products_from_cart(conn: &mut MultiplexedConnection) -> Result<Vec<MyProductInCart>, String> {
        let res: std::collections::HashMap<String, String> = { conn.hgetall("my_cart").await.map_err(|e: redis::RedisError| e.to_string())?};
        let mut products: Vec<MyProduct> = vec![];
        // Сначала собираем базовую информацию о продуктах
        for (key, name) in &res {
            // Проверяем, что это основной ключ продукта (без суффиксов)
            if key.starts_with("prod:") && !key.contains('-') {
                if let Some(id_str) = key.strip_prefix("prod:") {
                    if let Ok(id) = id_str.parse::<u64>() {
                        products.push(MyProduct { name: name.clone(), cost: 0, id: id });
                    }
                }
            }
        }
        // Теперь собираем полную информацию с количеством и стоимостью
        let mut products_in_cart: Vec<MyProductInCart> = Vec::new();
        for product in products {
            let count_key: String = format!("prod:{}-count", product.id);
            let count: u64 = res.get(&count_key).and_then(|s: &String| s.parse::<u64>().ok()).unwrap_or(0);
            let cost_key: String = format!("prod:{}-cost", product.id);
            let cost: u64 = res.get(&cost_key).and_then(|s: &String| s.parse::<u64>().ok()).unwrap_or(0);
            products_in_cart.push(MyProductInCart { product: MyProduct { name: product.name, cost: cost, id: product.id }, count: count });
        }
        Ok(products_in_cart)
    }

    pub async fn increment_product_count(product_id: u64, increment_by: u64, conn: &mut MultiplexedConnection) -> Result<u64, String> {
        // Используем HINCRBY для атомарного изменения
        let new_count: f64 = conn.hincr("my_cart", &format!("prod:{}-count", product_id), increment_by).await
            .map_err(|e: redis::RedisError| e.to_string())?;
        if new_count < 0.into() {
            return Err("Количество не может быть отрицательным".to_string());
        }
        Ok(new_count as u64)
    }

    pub async fn decrement_product_count(product_id: u64, increment_by: u64, conn: &mut MultiplexedConnection) -> Result<u64, String> {
        let new_count: f64 = conn.hincr("my_cart", &format!("prod:{}-count", product_id), -1 * i128::from(increment_by)).await
            .map_err(|e: redis::RedisError| e.to_string())?;
        if new_count < 0.into() {
            return Err("Количество не может быть отрицательным".to_string());
        }
        Ok(new_count as u64)
    }
}

// Task 6 
pub struct MyNotificationService {}

impl MyNotificationService {
    pub async fn init(conn: &mut MultiplexedConnection) -> Result<(), String> {
        conn.xgroup_create_mkstream("my_notifications", "default", "0").await.map_err(|e: redis::RedisError| e.to_string())?;
        Ok(())
    }

    pub async fn push(notification: MyNotification, conn: &mut MultiplexedConnection) -> Result<(), String> {
        let fields: Vec<(&str, String)> = vec![ 
            ("author", notification.author.clone()), 
            ("text", notification.text.clone()) 
        ];
        let _ = conn.xadd("my_notifications", "*", &fields).await.map_err(|e: redis::RedisError| e.to_string())?;
        Ok(())
    }
     
    pub async fn listen_easy(conn: &mut MultiplexedConnection) -> Result<Vec<(String, String)>, String> {
        let result: Option<StreamReadReply> = { conn
            .xread(&["notifications"], &["$"])
            .await
            .map_err(|e: redis::RedisError| e.to_string())? };
        
        let Some(result) = result else {
            return Ok(Vec::new()); // пустой результат - это нормально
        };
        let mut messages: Vec<(String, String)> = Vec::new();
        // StreamReadReply содержит поле keys: Vec<StreamKey>
        for stream_key in result.keys {
            // StreamKey содержит поле ids: Vec<StreamId>
            for stream_id in stream_key.ids {
                let message_id: String = stream_id.id.clone();
                // StreamId содержит поле map: Vec<(String, String)>
                let author: Option<redis::Value> = stream_id.map.iter()
                    .find(|(k, _)| *k == "author")
                    .map(|(_, v)| v.clone());
                
                let text: Option<redis::Value> = stream_id.map.iter()
                    .find(|(k, _)| *k == "text")
                    .map(|(_, v)| v.clone());
                
                if let (Some(author), Some(text)) = (author, text) {
                    if let redis::Value::SimpleString(author) = author {
                        if let redis::Value::SimpleString(text) = text {
                            conn.xack("my_notifications", "default", &[message_id]).await
                                .map_err(|e: redis::RedisError| e.to_string())?;
                            messages.push((author, text));
                        }
                    }
                }
            }
        }
        Ok(messages)
    }
}
