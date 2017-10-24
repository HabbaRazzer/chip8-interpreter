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
        match key.get_keyval() {
            49  => p_system.set_key(0x1), // 1 -> 1
            50  => p_system.set_key(0x2), // 2 -> 2
            51  => p_system.set_key(0x3), // 3 -> 3
            52  => p_system.set_key(0xC), // 4 -> C
            113 => p_system.set_key(0x4), // Q -> 4
            119 => p_system.set_key(0x5), // W -> 5
            101 => p_system.set_key(0x6), // E -> 6
            114 => p_system.set_key(0xD), // R -> D
            97  => p_system.set_key(0x7), // A -> 7
            115 => p_system.set_key(0x8), // S -> 8
            100 => p_system.set_key(0x9), // D -> 9
            102 => p_system.set_key(0xE), // F -> E
            122 => p_system.set_key(0xA), // Z -> A
            120 => p_system.set_key(0x0), // X -> 0
            99  => p_system.set_key(0xB), // C -> B
            118 => p_system.set_key(0xF), // V -> F
            _ => ()
        }
        Inhibit(false)
    });

    // callback for delete event
    window.connect_delete_event(|_, _| {
        gtk::main_quit();
        Inhibit(false)
    });

    gtk::main();
}
