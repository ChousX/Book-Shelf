#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use book_shelf::*;
use eframe::egui;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 600.0)),

        ..Default::default()
    };
    let app = eframe::run_native(
        "Book Shelf",
        options,
        Box::new(|_| Box::new(App::test())),
    );
}
