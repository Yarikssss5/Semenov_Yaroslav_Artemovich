use my_models::{GetStudentsResult, MyCreateStudentResult, MyStudent};
use sqlx::{Pool, Postgres};
// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
mod database;


#[tauri::command]
async fn get_students() -> GetStudentsResult {
    let res: Result<Option<Vec<MyStudent>>, String> = database::redis_get_students().await;
    match res {
        Ok(Some(students)) => { 
            if students.len() <= 5 {
                let pool: &'static Pool<Postgres> = match database::get_pg_pool().await {
                Ok(pool) => pool,
                Err(e) => return From::from(Err(e.to_string()))
            };
            let students: Vec<MyStudent> = match MyStudent::get_all_students(pool).await {
                Ok(res) => res,
                Err(e) => return From::from(Err(e))
            };
            println!("Loaded students : ");
            for i in students.clone() {
                println!("{}", i.to_string());
            }
            println!("End of loaded students");
            // Сохраняем в Redis для будущих запросов
            match database::redis_set_students(students.clone()).await {
                Ok(_) => From::from(Ok(students)),
                Err(e) => From::from(Err(e))
            }
            } else {
                From::from(Ok(students)) 
            }
        },
        Ok(None) => {
            // Если в Redis нет данных, получаем из PostgreSQL
            let pool: &'static Pool<Postgres> = match database::get_pg_pool().await {
                Ok(pool) => pool,
                Err(e) => return From::from(Err(e.to_string()))
            };
            let students: Vec<MyStudent> = match MyStudent::get_all_students(pool).await {
                Ok(res) => res,
                Err(e) => return From::from(Err(e))
            };
            println!("Loaded students : ");
            for i in students.clone() {
                println!("{}", i.to_string());
            }
            println!("End of loaded students");
            // Сохраняем в Redis для будущих запросов
            match database::redis_set_students(students.clone()).await {
                Ok(_) => From::from(Ok(students)),
                Err(e) => From::from(Err(e))
            }
        }
        Err(e) => From::from(Err(e.to_string())),
    }
}

#[tauri::command]
async fn mine_crate_student(first_name: String, middle_name: String, last_name: String) -> MyCreateStudentResult {
    let pool: &Pool<Postgres> = match database::get_pg_pool().await {
        Ok(res) => res,
        Err(e) => return From::from(Err(e.to_string()))
    };
    let id: i64 = match MyStudent::get_next_id(pool).await {
        Ok(res) => res,
        Err(e) => return From::from(Err(e))
    };
    let st = MyStudent::new(first_name, middle_name, last_name, id);
    match st.save_to_db(pool).await {
        Ok(_) => From::from(Ok(st)),
        Err(e) => From::from(Err(e))
    }

}



#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![get_students, mine_crate_student])
        .setup(move |_a: &mut tauri::App| {
            tauri::async_runtime::spawn(async {
                match database::init_pg_pool().await {
                    Ok(res) => res,
                    Err(e) => {
                        println!("{e}")
                    }
                };
                match database::get_pg_pool().await {
                    Ok(res) => res,
                    Err(e) => {eprintln!("{e}"); return;}
                };
                match database::create_tables().await {
                    Ok(res) => res,
                    Err(e) => {
                        println!("{e}");
                    }
                };
                match database::init_redis_connection().await {
                    Ok(_) => println!("Redis connected !"),
                    Err(e) => {eprintln!("{e}"); return;}
                };
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
