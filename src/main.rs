mod model;
mod util;

extern crate core_foundation;
extern crate core_graphics;

use clap::Command;
use lazy_static::lazy_static;
use model::WindowInfo;
use std::fs;
use std::path::Path;
use std::process::Command as ProcessCommand;
use std::time::Duration;
use std::{sync::Mutex, thread::sleep};
use util::window::find_window_under_mouse;
use util::{focus_app, get_mouse_position};

lazy_static! {
    static ref LAST_WINDOW: Mutex<Option<WindowInfo>> = Mutex::new(None);
}

fn main_loop() {
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

fn start_service() {
    let app_name = "oxrise";
    let domain = "com.oxrise";
    let label = format!("{}.{}", domain, app_name);
    let home = std::env::var("HOME").expect("Could not get HOME env variable");
    let binary_dest = format!("/usr/local/bin/{}", app_name);
    let plist_dest = format!("{}/Library/LaunchAgents/{}.plist", home, label);

    let plist_content = format!(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple Computer//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>Label</key>
    <string>{label}</string>
    <key>ProgramArguments</key>
    <array>
        <string>{binary_dest}</string>
    </array>
    <key>RunAtLoad</key>
    <true/>
    <key>KeepAlive</key>
    <true/>
    <key>StandardOutPath</key>
    <string>/tmp/{app_name}.out</string>
    <key>StandardErrorPath</key>
    <string>/tmp/{app_name}.err</string>
</dict>
</plist>
"#,
        label = label,
        binary_dest = binary_dest,
        app_name = app_name
    );

    let plist_dir = format!("{}/Library/LaunchAgents", home);
    fs::create_dir_all(&plist_dir).expect("Failed to create LaunchAgents directory");
    fs::write(&plist_dest, plist_content).expect("Failed to write plist file");

    let output = ProcessCommand::new("launchctl")
        .arg("load")
        .arg(&plist_dest)
        .output()
        .expect("Failed to execute launchctl load");

    if output.status.success() {
        println!("Service started successfully.");
    } else {
        eprintln!(
            "Failed to start service: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

fn stop_service() {
    let app_name = "oxrise";
    let domain = "com.oxrise";
    let label = format!("{}.{}", domain, app_name);
    let home = std::env::var("HOME").expect("Could not get HOME env variable");
    let plist_dest = format!("{}/Library/LaunchAgents/{}.plist", home, label);

    let output = ProcessCommand::new("launchctl")
        .arg("unload")
        .arg(&plist_dest)
        .output()
        .expect("Failed to execute launchctl unload");

    if output.status.success() {
        println!("Service stopped successfully.");
    } else {
        eprintln!(
            "Failed to stop service: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    if Path::new(&plist_dest).exists() {
        fs::remove_file(&plist_dest).expect("Failed to remove plist file");
        println!("Plist file removed.");
    }
}

fn main() {
    let matches = Command::new("oxrise")
        .version("0.1-RC1")
        .author("Theo Souchon <souchontheo24@gmail.com>")
        .about("A macOS window management tool inspired by X11, written in Rust")
        .subcommand(
            Command::new("--start-service")
                .about("Installs and starts OxRise as a background service"),
        )
        .subcommand(
            Command::new("--stop-service")
                .about("Stops and uninstalls the OxRise background service"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("--start-service", _sub_m)) => {
            start_service();
        }
        Some(("--stop-service", _sub_m)) => {
            stop_service();
        }
        _ => {
            println!("Running OxRise in interactive mode...");
            main_loop();
        }
    }
}
