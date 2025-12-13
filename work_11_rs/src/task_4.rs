use my_models::{MyTask, MyTaskPriority};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, MouseEvent, TargetCast, UseStateHandle, function_component, html, use_effect_with, use_state};

use crate::app::{my_add_task, my_get_sorted_by_score_tasks, my_remove_task};


#[function_component(Task4)]
pub fn task_4() -> Html {
    let task_status: UseStateHandle<MyTaskPriority> = use_state(|| MyTaskPriority::Common);
    let task: UseStateHandle<String> = use_state(|| String::new());
    let tasks: UseStateHandle<Vec<MyTask>> = use_state(|| vec![] as Vec<MyTask>);
    // web_sys::console::log_1(&"Loading tasks".into());
    use_effect_with(tasks.clone(), move |tasks: &UseStateHandle<Vec<MyTask>>| {
        let tasks: UseStateHandle<Vec<MyTask>> = tasks.clone();
        spawn_local(async move {
            let res: Result<Vec<MyTask>, String> = my_get_sorted_by_score_tasks().await;
            let Ok(temres) = res else {
                web_sys::console::log_1(&"Error : \n ".into());
                web_sys::console::log_1(&res.err().into());
                return;
            };
            tasks.set(temres);
        });
        || ()
    });
    web_sys::console::log_1(&"Loaded tasks".into());
    let create_task_callback: Callback<MouseEvent> = {
        let tasks: UseStateHandle<Vec<MyTask>> = tasks.clone();
        let task: UseStateHandle<String> = task.clone();
        let task_status: UseStateHandle<MyTaskPriority> = task_status.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let tasks: UseStateHandle<Vec<MyTask>> = tasks.clone();
            let task: UseStateHandle<String> = task.clone();
            let task_status: UseStateHandle<MyTaskPriority> = task_status.clone();
            spawn_local(async move {
                match my_add_task(MyTask { str: task.to_string(), priority: (*task_status).clone() }).await {
                    Ok(_) => {
                        web_sys::console::log_1(&"Task Created !".into());
                        let res: Result<Vec<MyTask>, String> = my_get_sorted_by_score_tasks().await;
                        let Ok(temres) = res else {
                            web_sys::console::log_1(&res.err().into());
                            return;
                        };
                        tasks.set(temres);
                    },
                    Err(e) => web_sys::console::log_1(&e.into()),
                };
            });
        })
    };
    web_sys::console::log_1(&"on_change_task_priority".into());
    let on_change_task_priority: Callback<Event> = {
        let task_status: UseStateHandle<MyTaskPriority> = task_status.clone();
        Callback::from(move |e: Event| {
            let target: Option<web_sys::HtmlElement> = e.target_dyn_into::<web_sys::HtmlElement>();
            let Some(el) = target else { return; };
            let Some(value) = el.node_value() else { return; };
            task_status.set(From::from(value));
        })
    };
    web_sys::console::log_1(&"on_change_task_priority".into());
    html!{
        <div>
            <div>
                <input type="text" onchange={
                    let task = task.clone();
                    Callback::from(move |e: Event| {
                        e.prevent_default();
                        let task = task.clone();
                        let input: HtmlInputElement = e.target_unchecked_into();
                        let value: String = input.value();
                        task.set(value);
                    })
                } value={task.to_string()} />
                <button onclick={create_task_callback} >{"+"}</button>
                <select name="task_priority" onchange={on_change_task_priority} id="task_priority">
                    <option value="Common" selected={true}>{"Common"}</option>
                    <option value="Emergancy">{"Emergancy"}</option>
                    <option value="Expired">{"Expired"}</option>
                </select>
            </div>
            <div>
                <h1>{"Задачи :"}</h1>
                {
                    for tasks.iter().map(|tek_task| {
                        html!{
                            <span>{tek_task.clone().to_string()}<button onclick={
                                let task = tek_task.clone();
                                let tasks: UseStateHandle<Vec<MyTask>> = tasks.clone();
                                Callback::from(move |e: MouseEvent| {
                                    e.prevent_default();
                                    let tasks: UseStateHandle<Vec<MyTask>> = tasks.clone();
                                    let task = task.clone();
                                    spawn_local(async move {
                                        match my_remove_task(task).await {
                                            Ok(_) => {
                                                web_sys::console::log_1(&"Task Removed !".into());
                                                let res: Result<Vec<MyTask>, String> = my_get_sorted_by_score_tasks().await;
                                                let Ok(temres) = res else {
                                                    web_sys::console::log_1(&res.err().into());
                                                    return;
                                                };
                                                tasks.set(temres);
                                            },
                                            Err(e) => web_sys::console::log_1(&e.into()),
                                        };
                                    });
                                })
                            } >{"X"}</button></span>
                        }
                    })
                }
            </div>
        </div>
    }
}