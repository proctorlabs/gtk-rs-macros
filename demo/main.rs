use gio::prelude::*;
use gtk::prelude::*;
use std::env::args;

use gtk::{ApplicationWindow, Builder, BuilderExtManual};

#[macro_use]
extern crate gtk_macros;

glade_app! {
    name: AppName,
    file: "test.glade",
    connect: {
    }
}

fn main() {
    AppName::start();
    //gtk::BuilderExtManual::connect_signals("test");
}
