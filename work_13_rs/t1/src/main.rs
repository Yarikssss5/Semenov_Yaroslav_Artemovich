use gtk::prelude::*;
use gtk::{ButtonsType, MessageDialog, MessageType, Window};

fn main() {
    gtk::init().expect("Не удалось инициализировать GTK");
    let dialog = MessageDialog::new(
        None::<&Window>,
        gtk::DialogFlags::MODAL,
        MessageType::Info,
        ButtonsType::Ok,
        "Привет! Меня зовут Ярослав",
    );
    dialog.set_title("Мое приветствие");
    dialog.set_default_size(250, 100);
    dialog.run();
    dialog.close();
}
