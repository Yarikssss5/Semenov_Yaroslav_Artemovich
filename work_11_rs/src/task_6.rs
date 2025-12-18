use my_models::MyNotification;
use wasm_bindgen_futures::spawn_local;
use yew::{Callback, Html, MouseEvent, UseStateHandle, function_component, html, use_state};

use crate::app::my_get_notifications;


#[function_component(Task6)]
pub fn task_6() -> Html {
    let notifications: UseStateHandle<Vec<MyNotification>> = use_state(|| vec![]);
    let load_notifications_callback: Callback<MouseEvent> = {
        let notifications: UseStateHandle<Vec<MyNotification>> = notifications.clone();
        Callback::from(move |e: MouseEvent| {
            let notifications: UseStateHandle<Vec<MyNotification>> = notifications.clone();
            e.prevent_default();
            spawn_local(async move {
                match my_get_notifications().await {
                    Ok(res) => notifications.set(res),
                    Err(e) => web_sys::console::log_1(&e.into()),
                };
            });
        })
    };
    html!{
        <div>
            <button onclick={load_notifications_callback} >{" Load notifications "}</button>
            <div>
                {
                    for notifications.iter().map(|el: &MyNotification| {
                        html!{
                            <div>
                                <h5>{el.author.to_string()}</h5>
                                <h5>{el.text.to_string()}</h5>
                            </div>
                        }
                    })
                }
            </div>
        </div>
    }
}