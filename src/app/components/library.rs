use std::path::PathBuf;

use eframe::epaint::ahash::HashMap;
use egui_extras::RetainedImage;

use super::{egui, App, AppEvent};
pub fn library(app: &App, ctx: &egui::Context) -> AppEvent {
    let mut image_path = Vec::new();
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for book in app.book_list.iter() {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    if let Some(path) = &book.image_path{
                        if let Some(image) = app.images.get(path){
                            image.show(ui);
                        } else {
                            image_path.push(path.clone());
                        }
                    } else 
                    {
                        app.images.get(&PathBuf::from("no_pic.png")).unwrap().show(ui);
                    }
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.label(&book.title);
                        if let Some(author) = &book.authour {
                            ui.label(format!("Author: {}", author));
                        }
                        if let Some(narrator) = &book.narrator {
                            ui.label(format!("Narrator: {}", narrator));
                        }
                        ui.separator();
                    });
                });
            }
        });
    });
    if image_path.is_empty(){
        AppEvent::None
    } else {
        AppEvent::AddImages(image_path)
    }
}
