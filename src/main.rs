extern crate gtk;

use gtk::traits::*;
use gtk::{signal, widgets};
use gtk::signal::TreeViewSignals;

fn activate() {
    let window = widgets::Window::new(gtk::WindowType::Toplevel).unwrap();
    window.set_title("Title".as_ref());
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(200, 200);
    window.show_all();
}

fn main() {
    match gtk::init() {
        Ok(_) => (),
        Err(e) => {
            panic!("Error initializing gtk: {:?}", e);
        }
    }

    let window = widgets::Window::new(gtk::WindowType::Toplevel).unwrap();
    window.set_title("Title".as_ref());
    window.set_window_position(gtk::WindowPosition::Center);
    window.set_default_size(200, 200);
    window.show_all();

    gtk::main();

    return;
}
