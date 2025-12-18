use my_models::{MyProduct, MyProductInCart, MyU64, my_rub};
use wasm_bindgen_futures::spawn_local;
use yew::{Callback, Html, MouseEvent, TargetCast, UseStateHandle, function_component, html, use_state};

use crate::app::{my_add_product, my_decrement_product_count, my_get_all_products_from_cart, my_increment_product_count, my_remove_product};

#[function_component(Task5)]
pub fn task_5() -> Html {
    let shop: UseStateHandle<Vec<MyProductInCart>> = use_state(|| vec![
        MyProductInCart { product: MyProduct { name: "Carrot".to_string(), cost: my_rub!(100), id: 0 }, count: 10 },
        MyProductInCart { product: MyProduct { name: "Apple".to_string(), cost: my_rub!(12), id: 1 }, count: 10 },
    ]);
    let selected_product: UseStateHandle<Option<MyProductInCart>> = use_state(|| None);
    let user_cart: UseStateHandle<Vec<MyProductInCart>> = use_state(|| vec![]);
    let on_select_product_callback: Callback<MouseEvent> = {
        let product: UseStateHandle<Option<MyProductInCart>> = selected_product.clone();
        let user_cart: UseStateHandle<Vec<MyProductInCart>> = user_cart.clone();
        let shop: UseStateHandle<Vec<MyProductInCart>> = shop.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let user_cart: UseStateHandle<Vec<MyProductInCart>> = user_cart.clone();
            let target: Option<web_sys::HtmlElement> = e.target_dyn_into::<web_sys::HtmlElement>();
            let Some(button) = target else { return; }; // HtmlElement
            let Some(product_id) = button.get_attribute("product_id") else { return; }; // String
            let res: Vec<MyProductInCart> = shop.iter().filter(|item: &&MyProductInCart| { 
                    if item.product.id.clone().to_string() == product_id { true } else { false } })
                .map(|item: &MyProductInCart| { item.clone() }).collect();
            if res.len() == 0 { return; }
            product.set(Some(res[0].clone()));
            spawn_local(async move {
                match my_add_product(res[0].clone()).await {
                    Ok(_) => match my_get_all_products_from_cart().await {
                        Ok(res) => user_cart.set(res),
                        Err(e) => web_sys::console::log_1(&e.into())
                    },
                    Err(e) => web_sys::console::log_1(&e.into())
                };
            });
            // shop.set(shop.iter().cloned().chain(vec![res[0].clone()]).collect());
            // user_cart.set(user_cart.iter().cloned().chain(vec![res[0].clone()]).collect());
        })
    };
    let remove_product_from_cart_callback: Callback<MouseEvent> = {
        let cart: UseStateHandle<Vec<MyProductInCart>> = user_cart.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let cart: UseStateHandle<Vec<MyProductInCart>> = cart.clone();
            let target: Option<web_sys::HtmlElement> = e.target_dyn_into::<web_sys::HtmlElement>();
            let Some(button) = target else { return; }; // HtmlElement
            let Some(product_id) = button.get_attribute("product_id") else { return; }; // String
            let Ok(product_id) = MyU64::try_from(product_id) else { return; };
            spawn_local(async move {
                match my_remove_product(From::from(product_id)).await {
                    Ok(_) => {
                        match my_get_all_products_from_cart().await {
                            Ok(res) => cart.set(res),
                            Err(e) => web_sys::console::log_1(&e.into()),
                        };
                    },
                    Err(e) => web_sys::console::log_1(&e.into()),
                };
            });
        })
    };
    let decrement_product_callback: Callback<MouseEvent> = {
        let cart: UseStateHandle<Vec<MyProductInCart>> = user_cart.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let cart: UseStateHandle<Vec<MyProductInCart>> = cart.clone();
            let target: Option<web_sys::HtmlElement> = e.target_dyn_into::<web_sys::HtmlElement>();
            let Some(button) = target else { return; }; // HtmlElement
            let Some(product_id) = button.get_attribute("product_id") else { return; }; // String
            let Ok(product_id) = MyU64::try_from(product_id) else { return; };
            spawn_local(async move {
                match my_decrement_product_count(From::from(product_id), 1).await {
                    Ok(_) => match my_get_all_products_from_cart().await {
                        Ok(res) =>  cart.set(res),
                        Err(e) => web_sys::console::log_1(&e.into()),
                    },
                    Err(e) => web_sys::console::log_1(&e.into()),
                };
            });
        })
    };
    let increment_product_callback: Callback<MouseEvent> = {
        let cart: UseStateHandle<Vec<MyProductInCart>> = user_cart.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let cart: UseStateHandle<Vec<MyProductInCart>> = cart.clone();
            let target: Option<web_sys::HtmlElement> = e.target_dyn_into::<web_sys::HtmlElement>();
            let Some(button) = target else { return; }; // HtmlElement
            let Some(product_id) = button.get_attribute("product_id") else { return; }; // String
            let Ok(product_id) = MyU64::try_from(product_id) else { return; };
            spawn_local(async move {
                match my_increment_product_count(From::from(product_id), 1).await {
                    Ok(_) => match my_get_all_products_from_cart().await {
                        Ok(res) => cart.set(res),
                        Err(e) => web_sys::console::log_1(&e.into()),
                    },
                    Err(e) => web_sys::console::log_1(&e.into()),
                };
            });
        })
    };
    let total: UseStateHandle<String> = use_state(|| String::new());
    let update_user_cart_callback: Callback<MouseEvent> = {
        let cart: UseStateHandle<Vec<MyProductInCart>> = user_cart.clone();
        let total: UseStateHandle<String> = total.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let cart: UseStateHandle<Vec<MyProductInCart>> = cart.clone();
            let total: UseStateHandle<String> = total.clone();
            spawn_local(async move {
                match my_get_all_products_from_cart().await {
                    Ok(res) => {
                        let mut sum = 0u64;
                        for i in res.clone() {
                            sum += i.count * i.product.cost;
                        }
                        web_sys::console::log_1(&format!("{},{}", sum / 100, sum % 100).to_string().into());
                        // web_sys::console::log_1(&JsValue::from_str(&format!("{},{}", sum / 100, sum % 100)));
                        total.set(format!("{},{}", sum / 100, sum % 100));
                        cart.set(res); 
                    },
                    Err(e) => web_sys::console::log_1(&e.into()),
                };
            });
        })
    };
    html! {
        <div>
            // Каталог
            <div>
                <h1>{"Товары :"}</h1>
                <div>
                    {
                        for shop.iter().map(|product: &MyProductInCart| {
                            html!{
                                <div>
                                    <h3>{product.product.name.clone()}</h3>
                                    {
                                        if (*user_cart).clone().contains(&product) {
                                            html! (
                                                <button product_id={product.product.id.clone().to_string()} 
                                                    onclick={on_select_product_callback.clone()} >{"+"}</button>
                                            )
                                        } else {
                                            html!(
                                                <div>
                                                    <button product_id={product.product.id.clone().to_string()}
                                                    onclick={increment_product_callback.clone()} >{"+"}</button>
                                                    <button product_id={product.product.id.clone().to_string()} 
                                                        onclick={decrement_product_callback.clone()}>{"-"}</button>
                                                    <button product_id={product.product.id.clone().to_string()}
                                                        onclick={remove_product_from_cart_callback.clone()}>{"Remove from cart"}</button>
                                                         <button product_id={product.product.id.clone().to_string()} 
                                                    onclick={on_select_product_callback.clone()} >{"Add product"}</button>
                                                </div>  
                                            )
                                        }
                                    }
                                </div>
                            }
                        })
                    }
                </div>
            </div>
            // Корзина пользователя
            <div>
                <h1>{"Корзина"}<button onclick={update_user_cart_callback.clone()}>{"Обновить карзину"}</button></h1>
                <div><h1>{format!("Итого : {}", &*total)}</h1></div>
                <div>
                    {
                        for user_cart.iter().map(|product: &MyProductInCart| {
                            html!{
                                <div>
                                    <h3>{product.clone().product.name.to_string() + "  |  " + &product.clone().count.to_string()}</h3>
                                    <div>
                                        <button product_id={product.product.id.clone().to_string()}
                                            onclick={increment_product_callback.clone()}>{"+"}</button>
                                        <button product_id={product.product.id.clone().to_string()}
                                            onclick={decrement_product_callback.clone()}>{"-"}</button>
                                    </div>
                                </div>
                            }
                        })
                    }
                </div>
            </div>
        </div>
    }
}
