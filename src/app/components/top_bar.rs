use super::{egui, App, AppEvent};
pub fn top_bar(app: &App, ctx: &egui::Context) -> AppEvent {
    let mut output = AppEvent::None;
    egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            if ui.button("⚙").clicked() {
                output = AppEvent::ToggleOption;
            }
            ui.label(app.state.to_string());
        });
    });
    output
}
