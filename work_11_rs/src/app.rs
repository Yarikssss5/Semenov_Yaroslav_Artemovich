use my_models::{Friend, MyAction, MyFileParsed, MyProductInCart, MyTask};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use crate::{task_1::Task1, task_2::Task2, task_3::Task3, task_4::Task4, task_5::Task5};

#[wasm_bindgen]
extern "C" {
    // invoke without arguments
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

fn js_value_to_json_str(value: &JsValue) -> String {
    if value.is_null() {  return "null".to_string();  }
    if value.is_undefined() {   return "undefined".to_string();   }
    let result = js_sys::JSON::stringify(value);
    if let Ok(js_string) = result {
        if let Some(rust_string) = js_string.as_string() {
            return rust_string;
        }
    }
    // Если все остальное не сработало, пытаемся как простую строку
    if let Some(str_val) = value.as_string() {
        return str_val;
    }
    // Для отладки
    web_sys::console::warn_1(&"Failed to convert JsValue to string".into());
    "".to_string()
}

// Task 1 :

pub(crate) async fn my_remove_from_cache_file(file_name: String) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "fileName": file_name }))
        .map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let js_result: JsValue = invoke("my_remove_from_cache_file", args).await;
    if js_value_to_json_str(&js_result) == "null" {
        return Ok(());
    }
    match serde_wasm_bindgen::from_value::<Result<(), String>>(js_result) {
        Ok(res) => {
            match res {
                Ok(_) => Ok(()),
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e.to_string())
    }
}

pub(crate) async fn fetchall_cached_static_files() -> Result<Vec<String>, String> {
    let js_result: JsValue = invoke_without_args("get_all_static_files").await;
    web_sys::console::log_1(&js_result);
    if js_value_to_json_str(&js_result) == "[]" {
        return Ok(vec![]);
    }
    match serde_wasm_bindgen::from_value::<Vec<String>>(js_result) {
        Ok(res) => Ok(res),
        Err(e) => {
            web_sys::console::log_1(&"Desialisation error !".into());
            Err(e.to_string())}
    }
}

pub(crate) async fn my_cache_file(file: MyFileParsed) -> Result<String, String> {
    let file_name: String = file.name.clone();
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "file": file })).map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let result: JsValue = invoke("my_cache_file", args).await;
    if js_value_to_json_str(&result) == "null" {
        return Ok(file_name);
    }
    match serde_wasm_bindgen::from_value::<Result<(), String>>(result) {
        Ok(res) => {
            match res {
                Ok(_) => Ok(file_name),
                Err(e) => Err(e)
            }
        },
        Err(e) => Err(e.to_string()),
    }
}

// pub(crate) async fn my_get_static_file(file_name: String) -> Result<MyFileParsed, String> {
//     let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "filename": file_name }))
//         .map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
//     let result: JsValue = invoke("my_get_static_file", args).await;
//     web_sys::console::log_1(&result);
//     match serde_wasm_bindgen::from_value::<MyFileParsed>(result) {
//         Ok(res) => Ok(res),
//         Err(e) => Err(e.to_string()),
//     }
// }

pub(crate) async fn my_save_file_from_redis_in_local(file_name: String) -> Result<String, String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "filename": file_name }))
        .map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let result: JsValue = invoke("my_save_file_from_redis_in_local", args).await;
    web_sys::console::log_1(&result);
    match serde_wasm_bindgen::from_value::<Result<Option<String>, String>>(result) {
        Ok(res) => match res {
                Ok(res) => match res {
                    Some(val) => Ok(val),
                    None => Err("".to_string())
                },
                Err(e) => Err(e)
            },
        Err(e) => Err(e.to_string()),
    }
}

// Task 2 :
pub(crate) async fn my_insert_action(action: MyAction) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "action": action })).map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let res: JsValue = invoke("my_insert_action", args).await;
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "null" {
        return Ok(());
    }
    // web_sys::console::log_1(&res);
    match serde_wasm_bindgen::from_value(res) {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}

pub(crate) async fn my_remove_action(action: MyAction) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "action": action })).map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let res: JsValue = invoke("my_remove_action", args).await;
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "null" {
        return Ok(());
    }
    match serde_wasm_bindgen::from_value(res) {
        Ok(res) => res,
        Err(e) => Err(e.to_string())
    }
}

pub(crate) async fn my_get_all_actions() -> Result<Vec<MyAction>, String> {
    let res: JsValue = invoke_without_args("my_get_all_actions").await;
    web_sys::console::log_1(&res);
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "[]" {
        return Ok(vec![]);
    }
    if let Ok(value) = serde_json::from_str::<Vec<MyAction>>(&res_str.clone()){
        Ok(value)
    } else {
        Err("This no vec  ".to_string() + &res_str.to_string())
    }
}

// Task 3
pub(crate) async fn my_load_friends(user_id: u64) -> Result<Vec<Friend>, String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "userId": user_id }))
        .map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let res: JsValue = invoke("get_friends", args).await;
    web_sys::console::log_1(&"Getted value".into());
    web_sys::console::log_1(&res.clone().into());
    web_sys::console::log_1(&"Getted value".into());
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "[]" {
        return Ok(vec![]);
    }
    match serde_wasm_bindgen::from_value::<Vec<Friend>>(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string())
    }
}

pub(crate) async fn my_load_recommendations(user_id: u64) -> Result<Vec<Friend>, String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "userId": user_id }))
        .map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let res: JsValue = invoke("get_friend_recommendations", args).await;
    web_sys::console::log_1(&res.clone().into());
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "[]" {
        return Ok(vec![]);
    }
    match serde_json::from_str::<Vec<Friend>>(&res_str) {
        Ok(res) => Ok(res),
        Err(_) => Err(res_str) 
    }
}

pub(crate) async fn my_add_friend(user_id: u64, friend_id: u64) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "userId": user_id, "friendId": friend_id }))
        .map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let res: JsValue = invoke("add_friend", args).await;
    web_sys::console::log_1(&res.clone().into());
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "null" {
        Ok(())
    } else {
        Err(res_str)
    }
}

pub(crate) async fn my_remove_friend(user_id: u64, friend_id: u64) -> Result<Vec<bool>, String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "userId": user_id, "friendId": friend_id }))
        .map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let res: JsValue = invoke("remove_friend", args).await;
    web_sys::console::log_1(&res.clone());
    match serde_wasm_bindgen::from_value::<Vec<bool>>(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string())
    }
}

// Task 4 
pub(crate) async fn my_add_task(task: MyTask) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "task": task })).map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    web_sys::console::log_1(&"my_add_task".into());
    let res: JsValue = invoke("my_add_task", args).await;
    web_sys::console::log_1(&res.clone());
    web_sys::console::log_1(&res.clone());
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "null" {
        Ok(())
    } else {
        Err(res_str)
    }
}  

pub(crate) async fn my_remove_task(task: MyTask) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "task": task })).map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    web_sys::console::log_1(&"my_remove_task".into());
    let res: JsValue = invoke("my_remove_task", args).await;
    web_sys::console::log_1(&res.clone());
    // web_sys::console::log_1(&res.clone());
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "null" {
        Ok(())
    } else {
        Err(res_str)
    }
}

pub(crate) async fn my_get_sorted_by_score_tasks() -> Result<Vec<MyTask>, String> {
    web_sys::console::log_1(&"CALL my_get_sorted_by_score_tasks".into());
    let res: JsValue = invoke_without_args("my_get_sorted_by_score_tasks").await;
    web_sys::console::log_1(&res.clone());
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "[]" {
        return Ok(vec![]);
    }
    match serde_wasm_bindgen::from_value::<Vec<MyTask>>(res) {
        Ok(res) => Ok(res),
        Err(e) => Err(e.to_string())
    }
}

// Task 5 :

pub(crate) async fn my_add_product(product: MyProductInCart) -> Result<(), String> {
    let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "product": product })).map_err(|e| e.to_string())?;
    web_sys::console::log_1(&args.clone());
    let result = invoke("my_add_product", args).await;
    let res_str = js_value_to_json_str(&result);
    if "null" == res_str {
        return Ok(())
    }
    Err(res_str)
}

pub(crate) async fn my_remove_product(product_id: u64) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "productId": product_id })).map_err(|e: serde_wasm_bindgen::Error| e.to_string())?;
    let res: JsValue = invoke("my_remove_product", args).await;
    let res_str: String = js_value_to_json_str(&res);
    if "null" == res_str {
        Ok(())
    } else {
        Err(res_str)
    }
}

pub(crate) async fn my_get_all_products_from_cart() -> Result<Vec<MyProductInCart>, String> {
    let res: JsValue = invoke_without_args("my_get_all_products_from_cart").await;
    let res_str: String = js_value_to_json_str(&res);
    if res_str == "[]" {
        return Ok(vec![]);
    }
    match  serde_wasm_bindgen::from_value::<Vec<MyProductInCart>>(res) {
        Ok(res) => Ok(res),
        Err(_) => Err(res_str)
    }
}

pub(crate) async fn my_increment_product_count(product_id: u64, increment_by: u64) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "productId": product_id, "incrementBy": increment_by })).map_err(|e| e.to_string())?;
    let res: JsValue = invoke("my_increment_product_count", args).await;
    let res_str: String = js_value_to_json_str(&res);
    if "null" == res_str {
        Ok(())
    } else {
        Err(res_str)
    }
}

pub(crate) async fn my_decrement_product_count(product_id: u64, increment_by: u64) -> Result<(), String> {
    let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({ "productId": product_id, "incrementBy": increment_by })).map_err(|e| e.to_string())?;
    let res: JsValue = invoke("my_decrement_product_count", args).await;
    let res_str: String = js_value_to_json_str(&res);
    if "null" == res_str {
        Ok(())
    } else {
        Err(res_str)
    }    
}

async fn my_error() -> Result<(), String> {
    let res: JsValue = invoke_without_args("my_error").await;
    web_sys::console::log_1(&res.is_string().into());
    Ok(())
}

enum MyPages {
    Main,
    Task1,
    Task2,
    Task3,
    Task4,
    Task5,
    Task6
}

#[function_component(App)]
pub fn app() -> Html {
    let task_visible: UseStateHandle<MyPages> = use_state(|| MyPages::Main);
    let task_2: UseStateHandle<MyPages> = task_visible.clone();
    let task_3: UseStateHandle<MyPages> = task_visible.clone();
    let task_4: UseStateHandle<MyPages> = task_visible.clone();
    let task_5: UseStateHandle<MyPages> = task_visible.clone();
    let task_6: UseStateHandle<MyPages> = task_visible.clone();
    let back_to_main_callback: Callback<()> = {
        let task_v: UseStateHandle<MyPages> = task_visible.clone();
        Callback::from(move |_: ()| {
            task_v.set(MyPages::Main);
        })
    };
    match *task_visible.clone() {
        MyPages::Main=>html!{
            <main class="container"> 
                <button onclick={Callback::from(move|_:MouseEvent| task_visible.set(MyPages::Task1))}>{" Task 1 "}</button>
                <button onclick={Callback::from(move|_:MouseEvent| task_2.set(MyPages::Task2))}>{" Task 2 "}</button> 
                <button onclick={Callback::from(move|_:MouseEvent| task_3.set(MyPages::Task3))}>{" Task 3 "}</button> 
                <button onclick={Callback::from(move|_:MouseEvent| task_4.set(MyPages::Task4))}>{" Task 4 "}</button> 
                <button onclick={Callback::from(move|_:MouseEvent| task_5.set(MyPages::Task5))}>{" Task 5 "}</button> 
                <button onclick={Callback::from(move|_:MouseEvent| task_6.set(MyPages::Task6))}>{" Task 6 "}</button>
                <button onclick={Callback::from(move|_:MouseEvent| {
                    spawn_local(async move {
                        let _ = my_error().await;
                    });
                })} >{" My Error ! "}</button>
            </main>
        },
        MyPages::Task1 => html!{ <Task1 back_callback={back_to_main_callback.clone()} /> },
        MyPages::Task2 => html!( <Task2 /> ),
        MyPages::Task3 => html!( <Task3 /> ),
        MyPages::Task4 => html!( <Task4 /> ),
        MyPages::Task5 => html!( <Task5 /> ),
        MyPages::Task6 => todo!(),
    }
}
