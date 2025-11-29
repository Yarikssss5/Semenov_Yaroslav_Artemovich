use my_models::{GetStudentsResult, MyCreateStudentResult, MyStudent, my_safe_get_str_from_vec, my_split_string};
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;

    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_without_args(cmd: &str) -> JsValue;

    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub(crate) fn my_console_log(str: &str) {
    log(str);
}

#[function_component(App)]
pub fn app() -> Html {
    let students: UseStateHandle<Vec<MyStudent>> = use_state(|| vec![]);
    let err_state: UseStateHandle<String> = use_state(|| String::new());
    let student_name: UseStateHandle<String> = use_state(|| String::new());
    let load_students_callback: Callback<()> = {
        let students: UseStateHandle<Vec<MyStudent>> = students.clone();
        Callback::from(move |_: ()| {
            let students: UseStateHandle<Vec<MyStudent>> = students.clone();
            spawn_local(async move {
                let res: JsValue = invoke_without_args("get_students").await;
                match serde_wasm_bindgen::from_value::<GetStudentsResult>(res) {
                    Ok(res) => {
                        match res.res {
                            Ok(res) =>{my_console_log(&res.len().to_string()) ; students.set(res)},
                            Err(e) => my_console_log(&e),
                        }
                    },
                    Err(e) => {my_console_log("Error !"); my_console_log(&e.to_string()); },
                }
            });
        })
    };
    let create_student_callback: Callback<(String, String, String)> = {
        let err: UseStateHandle<String> = err_state.clone();
        Callback::from(move|(first_name, middle_name, last_name): (String, String, String)| {
            let err: UseStateHandle<String> = err.clone();
            spawn_local(async move {
                let args: JsValue = serde_wasm_bindgen::to_value(&serde_json::json!({
                    "firstName": first_name,
                    "middleName": middle_name,
                    "lastName": last_name
                }))
                    .map_err(|e: serde_wasm_bindgen::Error| {err.set(e.to_string()); return e.to_string();})
                    .unwrap();
                let res: JsValue = invoke("mine_crate_student", args).await;
                let _: MyStudent = match serde_wasm_bindgen::from_value::<MyCreateStudentResult>(res) {
                    Ok(res) => {
                        match res.res {
                            Ok(res) => res,
                            Err(e) => { my_console_log(&e); err.set(e); return;},
                        }
                    },
                    Err(e) =>{my_console_log(&e.to_string()); err.set(e.to_string()); return;},
                };
            });    
        })
    };
    let on_input_callback: Callback<InputEvent> = {
        let student_name: UseStateHandle<String> = student_name.clone();
        Callback::from(move |e: InputEvent| {
            let value: String = e.target_unchecked_into::<HtmlInputElement>().value();
            student_name.set(value)
        })
    };
    html!{
        <div>
            <div class="w-[100%] flex-col">
                <div class="w-[100%] text-center py-2 mt-1">
                    <input type="text" oninput={on_input_callback.clone()} 
                        value={student_name.to_string()} class="w-[90%] text-left border rounded-xl px-4 py-2" />
                </div>
                <div class="w-[100%] text-center py-2">
                    <button onclick={
                        let call = create_student_callback.clone();
                        Callback::from(move |_: MouseEvent| {
                            let splited: Vec<String> = my_split_string(student_name.to_string().clone());
                            let first_name: String = my_safe_get_str_from_vec(splited.clone(), 0);
                            let middle_name =  my_safe_get_str_from_vec(splited.clone(), 1);
                            let last_name =  my_safe_get_str_from_vec(splited.clone(), 2);
                            call.emit((first_name, middle_name, last_name));
                        })
                    } class="border rounded-xl px-4 py-2">{"Добавить пользователя"}</button>
                </div>
                <div class="w-[100%] text-center py-2">
                    <button onclick={
                        let call = load_students_callback.clone();
                        Callback::from(move |_: MouseEvent|  call.emit(())) 
                    } class="b-2 border rounded-xl px-4 py-2">{"Загрузить пользователей"}</button>
                </div>
            </div>
            <div class="flex-col ml-5 mt-2 mr-5 w-[95%] border">
                <div class="w-[95%] text-left ">
                    {
                        if students.len() > 0 {                
                            html!{<>
                                {for (*students).iter().map(|student: &MyStudent| html!{
                                    <div>
                                        <h1>{student.first_name.clone()} {"    "} {student.middle_name.clone()} {"    "} {student.last_name.clone()}</h1>
                                    </div>
                                })}
                            </>}
                        } else {
                            html!{
                                <div class="rounded-xl px-4 py-2">
                                    <h1>{"Нет студентов !"}</h1>
                                </div>
                            }
                        }
                    }
                </div>
            </div>
        </div>
    }
}
