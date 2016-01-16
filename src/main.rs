#![crate_type = "bin"]
#[macro_use]

extern crate gtk;
extern crate gdk;

use std::cell::Cell;
use std::env;
use std::error::Error;
use std::fs::File;
use std::fs;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use gtk::traits::*;
use gtk::{signal, widgets};
use gtk::signal::Inhibit;
use gtk::signal::TreeViewSignals;
use gtk::Orientation::{Vertical};

use gdk::enums::key;
use gdk::enums::modifier_type;

#[cfg(feature="search_entry")]
fn get_entry_field() -> gtk::SearchEntry {
    gtk::SearchEntry::new().unwrap_or_else(|| panic!("Unable to instantiate GTK::SearchEntry!"))
}

#[cfg(not(feature="search_entry"))]
fn get_entry_field() -> gtk::Entry {
    gtk::Entry::new().unwrap_or_else(|| panic!("Unable to instantiate GTK::Entry!"))
}

fn main() {
    gtk::init().unwrap_or_else(|_| panic!("Failed to initialize GTK."));

    // debug!("Major: {}, Minor: {}", gtk::get_major_version(), gtk::get_minor_version());

    let (window, entry) = {
        let window = gtk::widgets::Window::new(gtk::WindowType::Toplevel).unwrap();
        let container = gtk::widgets::Box::new(Vertical, 0).unwrap();
        let entry = get_entry_field();
        container.add(&entry);
        container.set_visible(true);
        window.add(&container);
        window.connect_delete_event(|_, _| {
           gtk::main_quit();
           Inhibit(false)
        });
        // Setup our window properties
        window.set_window_position(gtk::WindowPosition::Center);
        window.set_border_width(0);
        window.set_decorated(false);
        window.show_all();
        (window, entry)
    };


    // env_logger::init().unwrap_or_else(|x| panic!("Error initializing logger: {}", x));

    gtk::main();

    return;
}
