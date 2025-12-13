use wasm_bindgen::{JsCast, prelude::Closure};
use my_models::MyFileParsed;
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::{Callback, Event, Html, MouseEvent, Properties, TargetCast, UseStateHandle, function_component, html, use_state};
use base64::{engine::general_purpose, Engine as _};

use crate::app::{fetchall_cached_static_files, my_cache_file, my_remove_from_cache_file, my_save_file_from_redis_in_local};

#[derive(Debug, Properties, PartialEq)]
pub struct Task1Props {
    pub back_callback: Callback<()>
}

fn on_file_change(file: UseStateHandle<Option<MyFileParsed>>, input: HtmlInputElement) {
    let my_files: Option<web_sys::FileList> = input.files();
    let Some(my_files) = my_files else {
        web_sys::console::error_1(&"Нет выбранных файлов".into());
        return;
    };
    if my_files.length() == 0 {
        web_sys::console::error_1(&"Файл не выбран".into());
        file.set(None);
        return;
    }
    let Some(web_file) = my_files.get(0) else { 
        web_sys::console::error_1(&"Не удалось получить файл".into());
        file.set(None);
        return;
    };
    let name: String = web_file.name().clone();
    let size: usize = web_file.size().clone() as usize;
    // Читаем файл как ArrayBuffer
    let file_reader: web_sys::FileReader = web_sys::FileReader::new().unwrap();
    let file_reader_clone: web_sys::FileReader = file_reader.clone();
    let file: UseStateHandle<Option<MyFileParsed>> = file.clone();
    let mime_type = web_file.type_();
    let onload: Closure<dyn FnMut(yew::ProgressEvent)> = Closure::wrap(Box::new(move |_: web_sys::ProgressEvent| {
        match file_reader_clone.result() {
            Ok(res) => {
                let uint8_array: Vec<u8> = js_sys::Uint8Array::new(&res).to_vec();
                let parsed_file: MyFileParsed = MyFileParsed {
                    name: name.clone(),
                    size: size.try_into().unwrap(),
                    data: general_purpose::STANDARD.encode(uint8_array),
                    mime_type: mime_type.clone()
                };
                file.set(Some(parsed_file));
                web_sys::console::log_1(&format!("Файл '{}' загружен, размер: {} байт", name, size).into());
            },
            Err(e) => web_sys::console::error_1(&e.into()),
        }
    }) as Box<dyn FnMut(_)>);
     // Устанавливаем обработчик и начинаем чтение
    file_reader.set_onload(Some(onload.as_ref().unchecked_ref()));
    file_reader.read_as_array_buffer(&web_file).unwrap();
    // Забываем closure, чтобы он не был удален
    onload.forget();
}

#[function_component(Task1)]
pub(crate) fn task_1(props: &Task1Props) -> Html {
    let back_callback: Callback<()> = props.back_callback.clone();
    let files: yew::UseStateHandle<Vec<String>> = use_state(|| vec![]);
    let current_file: UseStateHandle<Option<MyFileParsed>> = use_state(|| None);
    let file_two: UseStateHandle<Option<MyFileParsed>> = current_file.clone();
    let file_url: UseStateHandle<String> = use_state(|| String::new());
    let file_list_update_callback: Callback<MouseEvent> = {
        // Клонируем для использования внутри асинхронного блока
        let files: yew::UseStateHandle<Vec<String>> = files.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let files: yew::UseStateHandle<Vec<String>> = files.clone();
            spawn_local(async move {
                match fetchall_cached_static_files().await {
                    Ok(fetched_files) => {
                        web_sys::console::log_1(&format!("Получено файлов: {}", fetched_files.len()).into());
                        files.set(fetched_files);
                    }
                    Err(e) => {
                        files.set(vec![]);
                        web_sys::console::error_1(&format!("Ошибка: {}", e).into());
                    }
                }
            });
        })
    };
    let select_file_to_cache_callback: Callback<Event> = {
        let file: UseStateHandle<Option<MyFileParsed>> = current_file.clone();
        Callback::from(move |e: Event| {
            e.prevent_default();
            let input: Option<HtmlInputElement> = e.target_dyn_into::<HtmlInputElement>();
            let input: HtmlInputElement = match input {
                Some(input) => input,
                None => {
                    web_sys::console::error_1(&"Не удалось получить input элемент".into());
                    return;
                }
            };
            on_file_change(file.clone(), input);
        })
    };
    let cache_file_callback: Callback<MouseEvent> = {
        let files: UseStateHandle<Vec<String>> = files.clone();
        let file: UseStateHandle<Option<MyFileParsed>> = file_two.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let file: UseStateHandle<Option<MyFileParsed>> = file.clone();
            let files: UseStateHandle<Vec<String>> = files.clone();
            let Some(file) = (*file).clone() else { return (); };
            spawn_local(async move {
                match my_cache_file(file.clone()).await {
                    Ok(_) => {
                        match fetchall_cached_static_files().await {
                            Ok(fetched_files) => {
                                web_sys::console::log_1(&format!("Получено файлов: {}", fetched_files.len()).into());
                                files.set(fetched_files);
                            }
                            Err(e) => {
                                files.set(vec![]);
                                web_sys::console::error_1(&format!("Ошибка: {}", e).into());
                            }        
                        }
                    },
                    Err(e) => web_sys::console::error_1(&e.into()),
                }
            });
        })
    };
    let remove_callback: Callback<MouseEvent> = {
        let files: UseStateHandle<Vec<String>> = files.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            // Получаем элемент кнопки
            let target: Option<web_sys::HtmlElement> = e.target_dyn_into::<web_sys::HtmlElement>();
            let Some(button) = target else { return; };
            // Получаем значение из data-file атрибута
            let Some(file_name) = button.get_attribute("file") else { return; };
            let files: UseStateHandle<Vec<String>> = files.clone();
            spawn_local(async move {
                match my_remove_from_cache_file(file_name).await {
                    Ok(_) => files.set(vec![]),
                    Err(e) => web_sys::console::error_1(&e.into())
                };
            });
        })
    };
    let load_file_callback: Callback<MouseEvent> = {
        let url: UseStateHandle<String> = file_url.clone();
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
            let url: UseStateHandle<String> = url.clone();
            let target: Option<web_sys::HtmlElement> = e.target_dyn_into::<web_sys::HtmlElement>();
            let Some(button) = target else { return; };
            // Получаем значение из file атрибута
            let Some(file_name) = button.get_attribute("file") else { return; };
            spawn_local(async move {
                let Ok(getted_url) = my_save_file_from_redis_in_local(file_name).await else { return; };
                url.set(getted_url);
            });
        })
    };
    html! {
        <div>
            <h1>{"Кэшированные статические файлы"}</h1>
            <p>{"Всего файлов: "} {files.len()}</p>
            <button onclick={file_list_update_callback.clone()} >{"Получить список файлов"}</button>
            { 
                if files.len() > 0 {
                    html! {
                        <ul>
                            { for files.iter().map(|file| html! {
                                <li>
                                    { file.clone().to_string().trim_start_matches("my_") }
                                    <button file={file.to_string().clone()} onclick={remove_callback.clone()} >{"X"}</button>
                                    <button file={file.to_string().clone()} onclick={load_file_callback.clone()} >{"Load From Redis !"}</button>
                                    {
                                        if !file_url.clone().to_string().is_empty() {
                                            html!( <a href={file_url.clone().to_string()} >{"Download File !"}</a> )
                                        } else {
                                            html!(<></>)
                                        }
                                    }
                                </li>
                            }) }
                        </ul>
                    }
                } else {
                    html!{
                        <h1>{"Нет закэшированных файлов"}</h1>
                    }
                }
            }
            <button onclick={Callback::from(move |_: MouseEvent| back_callback.emit(()))} >{"Вернутся на главную"}</button><br />
            <input onchange={select_file_to_cache_callback} type="file" name="file" id="file" /><br />
            <button onclick={cache_file_callback} >{"Кэшировать файл"}</button>
        </div>
    }
}