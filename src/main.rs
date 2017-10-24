mod system;
mod macros;
extern crate gtk;
extern crate gdk;

use gtk::prelude::*;
use gtk::{Window, WindowType};

use std::cell::{RefCell, RefMut};
use system::{System, KeyEventType};

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
    window.connect_key_press_event(clone!( system => move |_, key| {
        let mut p_system: RefMut<System> = wait_for_borrow!(system);

        p_system.handle_input(key.get_keyval(), KeyEventType::KeyPress);

        Inhibit(false)
    }));

    // callback for key release event
    window.connect_key_release_event(clone!( system => move |_, key| {
        let mut p_system: RefMut<System> = wait_for_borrow!(system);

        p_system.handle_input(key.get_keyval(), KeyEventType::KeyRelease);

        Inhibit(false)
    }));

    // callback for delete event
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
