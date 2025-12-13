use my_models::{MyAction, MyActionKind};
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, MouseEvent, TargetCast, UseStateHandle, function_component, html, platform::spawn_local, use_state};

use crate::app::{my_get_all_actions, my_insert_action, my_remove_action};

#[function_component(Task2)]
pub(crate) fn task_2() -> Html {
    let current_value: UseStateHandle<i64> = use_state(|| 0 as i64);
    let actions: UseStateHandle<Vec<MyAction>> = use_state(|| vec![] as Vec<MyAction>);
    let total_value = actions.iter().fold(0, |acc: i64, action: &MyAction| {
        match action.kind {
            MyActionKind::Minus => acc - action.value,
            MyActionKind::Plus => acc + action.value
        }
    });
    let on_plus_action_callback: Callback<MouseEvent> = {
        let current_value: UseStateHandle<i64> = current_value.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let current_value: UseStateHandle<i64> = current_value.clone();
            let action: MyAction = MyAction {kind: MyActionKind::Plus, value: *current_value};
            spawn_local(async move {
                let current_value: UseStateHandle<i64> = current_value.clone();
                match my_insert_action(action).await {
                    Ok(_) => current_value.set(1),
                    Err(e) => web_sys::console::log_1(&e.into()),
                };
            });
        })
    };
    let on_minus_action_callback: Callback<MouseEvent> = {
        let current_value: UseStateHandle<i64> = current_value.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let current_value: UseStateHandle<i64> = current_value.clone();
            let action: MyAction = MyAction {kind: MyActionKind::Minus, value: *current_value};
            spawn_local(async move {
                let current_value: UseStateHandle<i64> = current_value.clone();
                match my_insert_action(action).await {
                    Ok(_) => current_value.set(1),
                    Err(e) => web_sys::console::log_1(&e.into()),
                };
            });
        })
    };
    let load_actions_callback: Callback<MouseEvent> = {
        let actions: UseStateHandle<Vec<MyAction>> = actions.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let actions: UseStateHandle<Vec<MyAction>> = actions.clone();
            spawn_local(async move {
                match my_get_all_actions().await {
                    Ok(res) => actions.set(res),
                    Err(e) => {
                        web_sys::console::log_1(&"Error ".into());
                        web_sys::console::log_1(&e.into());
                    }
                }
            });
        })
    };
    let remove_action_callback: Callback<MouseEvent> = {
        let actions: UseStateHandle<Vec<MyAction>> = actions.clone();
        Callback::from(move |e: MouseEvent| {
            let actions: UseStateHandle<Vec<MyAction>> = actions.clone();
            let target: Option<web_sys::HtmlElement> = e.target_dyn_into::<web_sys::HtmlElement>();
            let Some(button) = target else { return; };
            let Some(action_str) = button.get_attribute("action") else { return; };
            let Ok(action) = serde_json::from_str::<MyAction>(&action_str) else {return;};
            spawn_local(async move {
                let actions: UseStateHandle<Vec<MyAction>> = actions.clone();
                match my_remove_action(action).await {
                    Ok(_) => {
                        let actions: UseStateHandle<Vec<MyAction>> = actions.clone();
                        match my_get_all_actions().await {
                            Ok(res) => actions.set(res),
                            Err(e) => web_sys::console::log_1(&e.into())
                        }
                    },
                    Err(e) => web_sys::console::log_1(&e.into())
                };
            });
        })
    };
    let on_total_value_change_callback: Callback<Event> = {
        let total_val: UseStateHandle<i64> = current_value.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            if let Some(input) = e.target_dyn_into::<HtmlInputElement>() {
                if let Ok(value) = input.value().parse::<i64>() {
                    total_val.set(value);
                }
            }
        })
    };
    html!{
        <div>
            <button onclick={load_actions_callback} >{"Load Actions"}</button>
            <button onclick={on_plus_action_callback} >{"+"}</button>
            <input onchange={on_total_value_change_callback} />
            <button onclick={on_minus_action_callback} >{"-"}</button>
            <ul>
                {
                    for actions.iter().map(|action: &MyAction| { 
                        let action_json = match serde_json::to_string(&action) {
                            Ok(json) => json,
                            Err(e) => {
                                web_sys::console::error_1(&format!("Failed to serialize action: {}", e).into());
                                "{}".to_string() // fallback
                            }
                        };
                        html!{
                        <li>
                            <div>
                                {action.value.clone().to_string()}
                                <button action={action_json} onclick={remove_action_callback.clone()} >{"X"}</button>
                            </div>
                        </li>
                    } })
                }
            </ul>
            <h1>{total_value.clone().to_string()}</h1>
        </div>
    }
}