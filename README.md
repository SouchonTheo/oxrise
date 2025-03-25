# Oxrise

Oxrise is a Rust-based project designed to interact with macOS windows and applications. It provides utilities to detect the window under the mouse cursor, retrieve window information, and programmatically focus on applications. This project leverages macOS-specific APIs and libraries to achieve its functionality.

## Features

- Detect the window currently under the mouse cursor.
- Retrieve detailed information about windows, such as position, size, and owner.
- Programmatically focus on macOS applications using Cocoa and AppleScript.
- Real-time tracking of mouse position and window focus.

## Dependencies

The project uses the following Rust crates:

- `core-graphics`: For interacting with macOS's Core Graphics framework.
- `core-foundation`: For working with macOS's Core Foundation framework.
- `cocoa`: For interfacing with macOS's Cocoa framework.
- `objc`: For Objective-C interoperability.
- `libc`: For low-level system calls.
- `lazy_static`: For managing static variables.

## How It Works

1. **Mouse Tracking**: The program continuously tracks the mouse position.
2. **Window Detection**: It identifies the window under the mouse cursor using macOS's Core Graphics APIs.
3. **Focus Management**: If the mouse moves to a new window, the program attempts to focus on the corresponding application using Cocoa or AppleScript.

## Usage

Run the program using `cargo run`. The console will display the current mouse position and information about the window under the cursor. If the mouse moves to a new window, the program will automatically focus on the corresponding application.

## Limitations

- This project is macOS-specific and relies on macOS APIs.
- It is a work in progress (WIP), and additional features or improvements may be added in the future.

## Contributing

Contributions are welcome! Feel free to open issues or submit pull requests to improve the project.

## License

This project is licensed under the MIT License.

---
**Note**: This project is still under development and may contain bugs or incomplete features.
