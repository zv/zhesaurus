#![crate_type = "bin"]
#[macro_use]

extern crate gtk;
extern crate gdk;
#[macro_use] extern crate log;

use std::env;
use std::error::Error;
use log::*;

use gtk::traits::*;
use gtk::{signal, widgets};
use gtk::signal::Inhibit;
use gtk::signal::TreeViewSignals;
use gtk::Orientation::{Vertical};

// Keyinfo
use gdk::enums::key;

use thesaurus::DefaultThesaurus;
use thesaurus::ThesaurusSource;

mod thesaurus;


#[cfg(feature="search_entry")]
fn get_entry_field() -> gtk::SearchEntry {
    gtk::SearchEntry::new().unwrap_or_else(|| panic!("Unable to instantiate GTK::SearchEntry!"))
}

#[cfg(not(feature="search_entry"))]
fn get_entry_field() -> gtk::Entry {
    gtk::Entry::new().unwrap_or_else(|| panic!("Unable to instantiate GTK::Entry!"))
}

fn key_press_handler(key_widget: gtk::Widget, key: &gdk::EventKey) -> Inhibit {
    let keyval = key.keyval as i32;
    println!("key pressed: {}", keyval);
    match keyval {
        key::Return => {
        }
    }
    return Inhibit(false)
}


fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    debug!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());


    let thesaurus = DefaultThesaurus::new();

    // Setup our window
    let (window, entry) = {
        let window = gtk::widgets::Window::new(gtk::WindowType::Toplevel).unwrap();
        let container = gtk::widgets::Box::new(Vertical, 0).unwrap();
        let entry = get_entry_field();
        window.add(&container);
        container.add(&entry);
        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });
        // Setup our window pro
        window.set_window_position(gtk::WindowPosition::Center);
        window.set_border_width(5);
        window.set_decorated(false);
        window.show_all();
        (window, entry)
    };

    window.connect_key_press_event(key_press_handler);

    // env_logger::init().unwrap_or_else(|x| panic!("Error initializing logger: {}", x));

    gtk::main();
}
