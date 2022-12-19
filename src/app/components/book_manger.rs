use std::path::PathBuf;

use super::{egui, App, AppEvent};
use crate::run;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct BookManger(String);
impl BookManger {
    pub fn show(&mut self, ctx: &egui::Context) -> AppEvent {
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.label(
                "Add a single book or many. Simply by inputing the path or root path repectivley",
            );
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    ui.add(egui::TextEdit::singleline(&mut self.0));
                    if ui.button("add").clicked() {
                        let save = self.0.clone();
                        self.0 = String::new();
                        return AppEvent::AddBooks(save);
                    } else {
                        AppEvent::None
                    }
                })
            })
            .inner
            .inner
    }
}
