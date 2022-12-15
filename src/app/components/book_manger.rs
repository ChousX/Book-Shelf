use super::{egui, App, AppEvent};

#[derive(Default)]
pub struct BookManger(String);
impl BookManger {
    pub fn show(&mut self, ctx: &egui::Context) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(
                "Add a single book or many. Simply by inputing the path or root path repectivley",
            );
            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                ui.add(egui::TextEdit::singleline(&mut self.0));
                if ui.button("add").clicked() {
                    // this is ware we need to get data out of books...
                }
            });
        });
    }
}
