use super::{egui, App, AppEvent};
pub fn library(app: &App, ctx: &egui::Context) -> AppEvent {
    egui::CentralPanel::default().show(ctx, |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            for book in app.book_list.iter() {
                ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                    if let Some(image) = &book.image {
                        image.show(ui);
                    } else {
                        app.default_image.show(ui);
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
    AppEvent::None
}
