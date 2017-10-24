mod system;
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Window, WindowType};

use std::cell::{RefCell, RefMut};
use system::System;

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let window = Window::new(WindowType::Toplevel);
    window.set_title("Chip8 Interpreter");
    window.set_default_size(800, 500);
    window.show_all();

    let system: RefCell<System> = RefCell::new(System::new());
    // callback for key press event
    window.connect_key_press_event(move |_, key| {
        let mut p_system: RefMut<System>;
        loop {
            let result = system.try_borrow_mut();
            if result.is_ok() {
                p_system = result.unwrap();
                break;
            }
        }

        p_system.handle_input(key.get_keyval());

        Inhibit(false)
    });

    // callback for delete event
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
