#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{collections::VecDeque, default, path::PathBuf};

use image::ImageBuffer;
use poll_promise::Promise;
use eframe::egui;


use book_shelf::*;


fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1000.0, 600.0)),

        ..Default::default()
    };
    let app = eframe::run_native(
        "Book Shelf",
        options,
        Box::new(|_| Box::new(App::default())),
    );
}



// struct MyApp{
//     name: String, age: u32,
// }

// impl Default for MyApp{
//     fn default() -> Self {
//         Self { name: "Jhon".into() , age: 50 }
//     }
// }

// impl eframe::App for MyApp{
//     fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
//         egui::CentralPanel::default().show(ctx, |ui| {
//             ui.heading("My egui application!");
//             ui.horizontal(|ui|{
//                 let name_lable = ui.label("Your Name: ");
//                 ui.text_edit_singleline(&mut self.name)
//                     .labelled_by(name_lable.id);
//             });
//             ui.add(egui::Slider::new(&mut self.age, 0..=120).text("age"));
//             if ui.button("click each year").clicked(){
//                 self.age += 1;
//             }
//             ui.label(format!("Hello '{}', age {}", self.name, self.age));
//         });
//     }
// }
