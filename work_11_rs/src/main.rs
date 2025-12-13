mod app;
mod task_1;
mod task_2;
mod task_3;
mod task_4;

use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
