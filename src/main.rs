mod model;
mod util;

extern crate cocoa;
extern crate core_foundation;
extern crate core_graphics;

use std::time::Duration;
use std::{sync::Mutex, thread::sleep};

use lazy_static::lazy_static;
use model::WindowInfo;
use util::window::find_window_under_mouse;
use util::{focus_app, get_mouse_position};

lazy_static! {
    static ref LAST_WINDOW: Mutex<Option<WindowInfo>> = Mutex::new(None);
}

fn main() {
    loop {
        print!("\x1B[2J\x1B[1;1H");

        let mouse_pos = get_mouse_position();
        println!("Position de la souris: ({}, {})", mouse_pos.x, mouse_pos.y);

        unsafe {
            if let Some(ref current_window) = *LAST_WINDOW.lock().unwrap() {
                if current_window.contains(mouse_pos.x, mouse_pos.y) {
                    println!("Still in the same window.");
                    sleep(Duration::from_millis(50));
                    continue;
                }
            }

            if let Some(new_window) = find_window_under_mouse(mouse_pos.x, mouse_pos.y) {
                println!(
                    "New window found: {} (x: {}, y: {}, width: {}, height: {})",
                    new_window.owner,
                    new_window.x,
                    new_window.y,
                    new_window.width,
                    new_window.height
                );
                focus_app(&new_window.owner);
                *LAST_WINDOW.lock().unwrap() = Some(new_window);
            } else {
                println!("No window found under mouse.");
            }
        }

        sleep(Duration::from_millis(50));
    }
}
