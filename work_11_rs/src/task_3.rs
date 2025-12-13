use my_models::Friend;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;

use crate::app::{my_load_friends, my_load_recommendations, my_add_friend, my_remove_friend};

#[function_component(Task3)]
pub fn task_3() -> Html {
    let friends: UseStateHandle<Vec<Friend>> = use_state(Vec::new);
    let recommendations: UseStateHandle<Vec<Friend>> = use_state(|| vec![]);
    // Загрузка друзей при первом рендере
    {
        let friends = friends.clone();
        use_effect_with((), move |_| {
            let friends = friends.clone();
            spawn_local(async move {
                if let Ok(fetched_friends) = my_load_friends(0).await {
                    friends.set(fetched_friends);
                }
            });
            || ()
        });
    }
    let load_recommendations: Callback<MouseEvent> = {
        let recommendations: UseStateHandle<Vec<Friend>> = recommendations.clone();
        Callback::from(move |_| {
            let lt: UseStateHandle<Vec<Friend>> = recommendations.clone();
            spawn_local(async move {
                let res: Result<Vec<Friend>, String> = my_load_recommendations(0).await;
                let Ok(recs) = res else {
                    web_sys::console::log_1(&res.err().unwrap().into());
                    return;
                };
                lt.set(recs);
            });
        })
    };
    html! {
        <div class="friends-container">
            <h2>{"Друзья"}</h2>
            <div class="friends-list">
                <h3>{"Список друзей"}</h3>
                <ul>
                    {for friends.iter().map(|friend| {
                        let friend_id = friend.id.clone();
                        html! {
                            <li key={friend.id}>
                                <span>{&friend.username}</span>
                                <button onclick={
                                    let friends = friends.clone();
                                    Callback::from(move |_| {
                                    // Удаление друга
                                    let friend_id = friend_id.clone();
                                    let friends = friends.clone();
                                    spawn_local(async move {
                                        let friends = friends.clone();
                                        match my_remove_friend(0, friend_id).await {
                                            Ok(_) => { web_sys::console::log_1(&"Friend Removed !".into());
                                            let result: Result<Vec<Friend>, String> = my_load_friends(0).await;
                                            let Ok(fetched_friends) = result else {
                                                web_sys::console::log_1(&result.err().unwrap().into());
                                                return;
                                            };
                                            friends.set(fetched_friends);
                                        },
                                            Err(e) => web_sys::console::log_1(&e.into()),
                                        };
                                    });
                                })}>
                                    {"Удалить"}
                                </button>
                            </li>
                        }
                    })}
                </ul>
            </div>
            <div class="recommendations">
                <h3>{"Рекомендации друзей"}</h3>
                <button onclick={load_recommendations}>{"Показать рекомендации"}</button>
                <ul>
                    {for recommendations.iter().map(|rec| {
                        if !friends.contains(&rec.clone()) {
                            let friend_id = rec.clone();
                            html! {
                                <li key={rec.id.to_string()}>
                                    <span>{format!("{}", rec.username).to_string()}</span>
                                    <button onclick={
                                        let friends: UseStateHandle<Vec<Friend>> = friends.clone();
                                        Callback::from(move |e: MouseEvent| {
                                        let friends: UseStateHandle<Vec<Friend>> = friends.clone();
                                        e.prevent_default();
                                        let friend_id = friend_id.clone();
                                        spawn_local(async move {
                                            let friends: UseStateHandle<Vec<Friend>> = friends.clone();
                                            match my_add_friend(0, friend_id.id.into()).await {
                                                Ok(_) => {
                                                    let result: Result<Vec<Friend>, String> = my_load_friends(0).await;
                                                    let Ok(fetched_friends) = result else {
                                                        web_sys::console::log_1(&result.err().unwrap().into());
                                                        return;
                                                    };
                                                    friends.set(fetched_friends);
                                                },
                                                Err(e) => web_sys::console::log_1(&e.into()),
                                            }
                                        });
                                    })}>
                                        {"Добавить в друзья"}
                                    </button>
                                </li>
                            }
                        } else {
                            html!()
                        }
                    })}
                </ul>
            </div>
        </div>
    }
}