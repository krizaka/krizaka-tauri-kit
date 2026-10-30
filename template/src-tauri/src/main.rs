// Prevents an extra console window on Windows in release; harmless on macOS.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    __slug___desktop_lib::run();
}
