use super::{view::View, app_state::AppState, AppEvent};
use eframe::{egui, epaint::ahash::HashMap};
use egui_extras::RetainedImage;

#[derive(Default)]
pub struct Options {
    pub visibility: View,
}
impl Options {
    const OPTION_LIST: [AppState; 3] = [
        AppState::Library,
        AppState::Preferences,
        AppState::BookManger,
    ];
    pub fn show(&self, ctx: &egui::Context) -> AppEvent {
        if self.visibility.is_visible() {
            egui::SidePanel::left("OptionsPanel")
                .show(ctx, |ui| {
                    for state in Self::OPTION_LIST {
                        if ui.button(state.to_string()).clicked() {
                            return AppEvent::SwitchState(state);
                        }
                    }
                    AppEvent::None
                })
                .inner
        } else {
            AppEvent::None
        }
    }
}