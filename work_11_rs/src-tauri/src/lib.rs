use base64::{Engine, engine::general_purpose};
use rand::Rng;
use redis::{aio::MultiplexedConnection, AsyncTypedCommands, RedisError};
use tauri::{App, AppHandle, Manager, State, async_runtime};
use tokio::{io::AsyncWriteExt, sync::{Mutex as TokioMutex, MutexGuard}};
use std::path;
use my_models::{Friend, MyAction, MyFileParsed, MyNotification, MyProductInCart, MyTask, native::{FriendService, MyNotificationService, MyProductCartServices, MyTaskService}};

fn get_exe_dir() -> Option<path::PathBuf> {
    match std::env::current_exe() {
        Ok(exe_path) => {
            // Получаем родительскую директорию (папку) исполняемого файла
            exe_path.parent().map(|p: &path::Path| p.to_path_buf())
        }
        Err(_) => None,
    }
}

#[allow(unused)]
pub(crate) struct AppState {
    redis_con: Option<MultiplexedConnection>,
    client: Option<redis::Client>
}

type WrappedState = TokioMutex<AppState>;

impl AppState {
    async fn check_connection(&mut self) -> Result<(), String> {
        match self.redis_con.take() {
            Some(mut con) => 
                match con.ping().await { 
                    Ok(_) => self.redis_con = Some(con),
                    Err(_) => println!("")
                },
            None => println!("Connection empty !")
        }
        Ok(())
    }

    fn get_connection_as_mut(&mut self) -> Result<&mut MultiplexedConnection, String> {
        self.redis_con
            .as_mut().ok_or_else(|| "Redis connection is not established".to_string())
    }
}
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

// Serialization test's :
#[tauri::command]
async fn my_error() -> Result<(), String> {
    let mut rng: rand::prelude::ThreadRng = rand::rng();
    match rng.random::<bool>() {
        true => Ok(()),
        false => Err("Error unexapted ecxaption !".to_string())
    }
}

// Task 1
// SET my_my.txt
#[tauri::command]
async fn my_cache_file(state: State<'_, WrappedState>, file: MyFileParsed) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let file_name: String = file.name.clone();
    let con: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let Ok(json) = serde_json::to_string(&MyFileParsed { size: file.size, name: file.name, data: file.data, mime_type:  file.mime_type}) else 
        {return Err(String::from("JSON Serialisation error !")) ;};
    con.set(format!("my_{}", file_name), json).await.map_err(|e: RedisError| e.to_string())?;
    Ok(())
}

// DEL "my_example_file.txt"
#[tauri::command]
async fn my_remove_from_cache_file(state: State<'_, WrappedState>, file_name: String) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let con: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    con.del(&file_name).await.map_err(|e: RedisError| e.to_string())?;
    Ok(())
}

// SCAN 0 MATCH "my_*"
#[tauri::command]
async fn get_all_static_files(state: State<'_, WrappedState>) -> Result<Vec<String>, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let mut files: Vec<String> = Vec::new();
    let keys: Vec<Result<String, RedisError>> = {
        let mut iter: redis::AsyncIter<'_, String> = conn.scan_match("my_*").await
            .map_err(|e: RedisError| e.to_string())?;
        let mut keys = Vec::new();
        while let Some(key) = iter.next_item().await {
            keys.push(key);
        }
        keys
    };
    for key in keys {
        let Ok(file) = key else {return Err(String::from(""));};
        let current_type: redis::ValueType = conn.key_type(file.clone()).await
            .map_err(|e: RedisError| e.to_string())?;
        if current_type == redis::ValueType::String {
            files.push(file);
        }
    }
    Ok(files)
}

// GET my_my.txt
#[tauri::command]
async fn my_get_static_file(
    state: State<'_, WrappedState>, 
    filename: String
) -> Result<MyFileParsed, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let Some(json_string) = conn.get(&filename).await.map_err(|e: RedisError| {println!("{e}"); e.to_string()})? else {
        return Err(String::from("File empty in redis !"));
    };
    // Десериализуем из JSON
    let file: MyFileParsed = serde_json::from_str(&json_string)
        .map_err(|e: serde_json::Error| e.to_string())?;
    Ok(file)
}

#[tauri::command]
async fn my_save_file_from_redis_in_local(state: State<'_, WrappedState>, filename: String) -> Result<Option<String>, String> {
    let Ok(parsedfile) =  my_get_static_file(state.clone(), filename).await else { return Err("File not found in redis !".to_string()); };
    let Some(exe_dir_path) = get_exe_dir() else {return Err("Can't get exe path !".to_string()) ;};
    let mut file_full_path: path::PathBuf = exe_dir_path.clone();
    file_full_path.push(parsedfile.name.trim_start_matches("my_").to_string());
    let str_file_full_path: String = file_full_path.to_string_lossy().to_string();
    println!("{}", str_file_full_path.clone());
    match tokio::fs::try_exists(file_full_path.clone()).await {
        Ok(true) => Err("File already exists !".to_string()),
        Ok(false) => {
            let Ok(decoded) = general_purpose::STANDARD.decode(parsedfile.data) else { return Err("Не декодировать из base64 содержимое файла".to_string()); };
            let Ok(mut file) = tokio::fs::File::create(file_full_path.clone()).await else { return Err("Can't open file !".to_string()); };
            file.write_all(&decoded).await.map_err(|e: std::io::Error| e.to_string())?;
            file.sync_all().await.map_err(|e: std::io::Error| e.to_string())?;
            let Ok(url) = url::Url::from_file_path(&file_full_path) else { return Err("Не удалось создать URL".to_string()); };
            Ok(Some(url.to_string()))
        },
        Err(e) => Err(format!("Error checking file existence: {}", e)),
    }
}

// Task 2 :
// LPUSH "my_work_2" action
#[tauri::command]
async fn my_insert_action(state: State<'_, WrappedState>, action: MyAction) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    conn.lpush("my_work_2", action).await.map_err(|e: RedisError| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn my_remove_action(state: State<'_, WrappedState>, action: MyAction) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    conn.lrem("my_work_2", 1, action).await.map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn my_get_all_actions(state: State<'_, WrappedState>) -> Result<Vec<MyAction>, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let res: Vec<String> = conn.lrange("my_work_2", 0, -1).await.map_err(|e: RedisError| e.to_string())?;
    let actions: Result<Vec<MyAction>, String> = res.into_iter().map(|json: String| {
            serde_json::from_str(&json).map_err(|e: serde_json::Error| format!("Failed to deserialize action: {}", e))
        }).collect();
    actions
}

// Task 3
#[tauri::command]
async fn add_friend(state: State<'_, WrappedState>, user_id: u64, friend_id: u64) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let service: FriendService = FriendService(user_id.to_string());
    service.add_friend(user_id, friend_id, conn).await
}

#[tauri::command]
async fn get_friend_recommendations(state: State<'_, WrappedState>, user_id: u64, limit: Option<usize>) -> Result<Vec<Friend>, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let service: FriendService = FriendService(user_id.to_string());
    let t_res: Vec<u64> = { service.get_friend_recommendations(user_id, limit, conn).await? };
    let mut friends : Vec<Friend> = vec![];
    for i in t_res {
        let friend: Friend = FriendService::get_profile_by_id(i, conn).await?;
        friends.push(friend);
    }
    Ok(friends)
}

#[tauri::command]
async fn remove_friend(state: State<'_, WrappedState>, user_id: u64, friend_id: u64) -> Result<(bool, bool), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let service: FriendService = FriendService::new(user_id.to_string());
    service.remove_friend(user_id, friend_id, conn).await
}

#[tauri::command]
async fn get_friends(state: State<'_, WrappedState>, user_id: u64) -> Result<Vec<Friend>, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let service: FriendService = FriendService::new(user_id.to_string());
    let res_t: Vec<u64> = service.get_friends(user_id, conn).await?;
    let mut fres: Vec<Friend> = vec![];
    for i in res_t {
        let friend: Friend = FriendService::get_profile_by_id(i, conn).await?;
        fres.push(friend);
    }
    Ok(fres)
}

async fn t3_init(state: State<'_, WrappedState>) -> Result<String, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let service: FriendService = FriendService::new("0".to_string());
    let friends: Vec<u64> = service.get_friends(0, conn).await?;
    if !friends.is_empty() {
        return Ok(format!("У пользователя 0 уже есть {} друзей: {:?}", friends.len(), friends));
    }
    // Создаем тестовых пользователей
    let test_users: Vec<(i32, &str)> = vec![
        (1, "Иван"),
        (2, "Мария"), 
        (3, "Алексей"),
        (4, "Екатерина"),
        (5, "Дмитрий"),
        (6, "Анна"),
        (7, "Сергей"),
        (8, "Ольга"),
        (9, "Павел"),
        (10, "Наталья"),
    ];
    // Создаем профили
    for (id, name) in &test_users {
        let profile_key: String = format!("user:{}:profile", id);
        conn.hset(&profile_key, "username", name).await.map_err(|e: RedisError| e.to_string())?;
    }
    // Добавляем друзей пользователю 0
    let friends_for_0: Vec<u64> = vec![1, 2, 3, 4, 5];
    for friend_id in &friends_for_0 {
        service.add_friend(0, *friend_id, conn).await?;
    }
    // Создаем связи для рекомендаций
    // Пользователь 1 дружит с 6, 7
    service.add_friend(1, 6, conn).await?;
    service.add_friend(1, 7, conn).await?;
    // Пользователь 2 дружит с 8, 9  
    service.add_friend(2, 8, conn).await?;
    service.add_friend(2, 9, conn).await?;
    // Пользователь 3 дружит с 10
    service.add_friend(3, 10, conn).await?;
    // Добавляем связи между друзьями пользователя 0
    service.add_friend(1, 2, conn).await?; // 1 дружит с 2
    service.add_friend(1, 4, conn).await?; // 1 дружит с 4
    // Проверяем рекомендации
    let recommendations: Vec<u64> = service.get_friend_recommendations(0, Some(10), conn).await?;
    Ok(format!(
        "✅ Dataset создан!\n\
         👤 У пользователя 0: {} друзей\n\
         📊 Всего пользователей: {}\n\
         🤝 Рекомендации для 0 ({} штук): {:?}",
        friends_for_0.len(),
        test_users.len(),
        recommendations.len(),
        recommendations
    ))
}

// Task 4
#[tauri::command]
async fn my_add_task(state: State<'_, WrappedState>, task: MyTask) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    MyTaskService::add_task(task, conn).await?;
    Ok(())
}

#[tauri::command]
async fn my_remove_task(state: State<'_, WrappedState>, task: MyTask) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    MyTaskService::remove_task(task, conn).await?;
    Ok(())
}

#[tauri::command]
async fn my_get_sorted_by_score_tasks(state: State<'_, WrappedState>) -> Result<Vec<MyTask>, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let res: Vec<MyTask> = MyTaskService::get_sorted_tasks_by_priority(conn).await?;
    Ok(res)
}

// Task 5
#[tauri::command]
async fn my_add_product(state: State<'_, WrappedState>, product: MyProductInCart) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    MyProductCartServices::add_product(product.product, product.count, conn).await?;
    Ok(())
}

#[tauri::command]
async fn my_remove_product(state: State<'_, WrappedState>, product_id: u64) -> Result<(), String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    MyProductCartServices::remove_product(product_id, conn).await?;
    Ok(())
}

#[tauri::command]
async fn my_get_all_products_from_cart(state: State<'_, WrappedState>) -> Result<Vec<MyProductInCart>, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    MyProductCartServices::get_all_products_from_cart(conn).await
}

#[tauri::command]
async fn my_increment_product_count(product_id: u64, increment_by: u64, state: State<'_, WrappedState>) -> Result<u64, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    MyProductCartServices::increment_product_count(product_id, increment_by, conn).await
}

#[tauri::command]
async fn my_decrement_product_count(product_id: u64, increment_by: u64, state: State<'_, WrappedState>) -> Result<u64, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    MyProductCartServices::decrement_product_count(product_id, increment_by, conn).await
}

// Task 6
#[tauri::command]
async fn my_get_notifications(state: State<'_, WrappedState>) -> Result<Vec<MyNotification>, String> {
    let mut state: MutexGuard<'_, AppState> = state.lock().await;
    state.check_connection().await?;
    let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
    let res: Vec<(String, String)> = MyNotificationService::listen_simple(conn).await?;
    println!("{:#?}", res);
    let final_res: Vec<MyNotification> = res.iter()
        .map(|(author, text)| {MyNotification { author: author.to_string(), text: text.to_string() }}).collect();
    Ok(final_res)
}


// Connecting to redis before start app :
async fn setup(app: &mut App) -> std::result::Result<(), Box<dyn std::error::Error>> {
    let client: redis::Client = redis::Client::open("redis://127.0.0.1/")?;
    let con: MultiplexedConnection = client.get_multiplexed_async_connection().await?;
    let app: &AppHandle = app.handle();
    let state: TokioMutex<AppState> = TokioMutex::new(AppState { redis_con: Some(con), client: Some(client) });
    app.manage(state);
    {
        match t3_init(app.state()).await {
            Ok(res) => println!("{}", res),
            Err(e) => eprintln!("{}", e)
        }; 
    };
    {
        let state: State<'_, WrappedState> = app.state();
        let mut state: MutexGuard<'_, AppState> = state.lock().await;
        state.check_connection().await?;
        let conn: &mut MultiplexedConnection = state.get_connection_as_mut()?;
        println!("Try init !");
        match MyNotificationService::init(conn).await {
            Ok(_) => {
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
                println!("INSERTED");
                MyNotificationService::push(MyNotificationService::generate_random(), conn).await?;
            },
            Err(e) => eprintln!("{}", e)
        };
    };
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app: &mut App| async_runtime::block_on(setup(app)))
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            my_error,
            // Task 1 : 
            my_cache_file, 
            get_all_static_files,
            my_remove_from_cache_file,
            my_get_static_file,
            my_save_file_from_redis_in_local,
            // Task 2 : 
            my_insert_action,
            my_remove_action,
            my_get_all_actions,
            // Task 3 :
            add_friend,
            remove_friend,
            get_friends,
            get_friend_recommendations,
            // Task 4 :
            my_add_task,
            my_remove_task,
            my_get_sorted_by_score_tasks,
            // Task 5 :
            my_add_product,
            my_remove_product,
            my_get_all_products_from_cart,
            my_increment_product_count,
            my_decrement_product_count,
            // Task 6 : 
            my_get_notifications
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}