mod app_state;
mod options;
mod view;

pub use app_state::AppState;
pub use options::Options;
pub use view::View;
use crate::*;

use eframe::egui;
use egui_extras::RetainedImage;

 
pub struct App {
    state: AppState,
    book_shelf: BookShelf,
    book_list: Vec<Book>,
    options: Options,
    input_faild: String,
    default_image: RetainedImage,
}

impl Default for App {
    fn default() -> Self {
        let mut book_shelf = BookShelf::default();
        book_shelf.add(Book {
            title: "test1".to_owned(),
            authour: Some("a".to_owned()),
            narrator: Some("n".to_owned()),
            ..Default::default()
        });
        book_shelf.add(Book {
            title: "test2".to_owned(),
            authour: Some("a2".to_owned()),
            narrator: Some("n2".to_owned()),
            ..Default::default()
        });
        book_shelf.add(Book {
            title: "test3".to_owned(),
            authour: Some("a3".to_owned()),
            narrator: Some("n3".to_owned()),
            ..Default::default()
        });
        let default_image =
             RetainedImage::from_image_bytes("default image", include_bytes!("no_pic.png")).unwrap();
        let mut out = Self {
            state: AppState::default(),
            book_shelf,
            book_list: Vec::default(),
            options: Options::default(),
            input_faild: String::default(),
            default_image,
        };
        out.book_list_title();
        out
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("topbar").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                if ui.button("⚙").clicked() {
                    self.options.visibility.toggle();
                }
                ui.label(self.state.to_string());
            });
        });
        self.handle(self.options.show(ctx));

        match self.state {
            AppState::Library => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    egui::ScrollArea::vertical().show(ui, |ui| {
                        for book in self.book_list.iter(){
                            ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
                                if let Some(image) = &book.image {
                                    image.show(ui);
                                } else {
                                    self.default_image.show(ui);
                                }
                                ui.with_layout(
                                    egui::Layout::top_down_justified(egui::Align::LEFT),
                                    |ui| {
                                        ui.label(&book.title);
                                        if let Some(author) = &book.authour {
                                            ui.label(format!("Author: {}", author));
                                        }
                                        if let Some(narrator) = &book.narrator {
                                            ui.label(format!("Narrator: {}", narrator));
                                        }
                                        ui.separator();
                                       
                                    },
                                );
                            });
                        }
                    });
                });
            }
            AppState::Preferences => {}
            AppState::Player => {}
            AppState::BookManger => {
                egui::CentralPanel::default().show(ctx, |ui|{
                    ui.label("Add a single book or many. Simply by inputing the path or root path repectivley");
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui|{
                        ui.add(egui::TextEdit::singleline(&mut self.input_faild));
                        if ui.button("add").clicked(){
                            // this is ware we need to get data out of books...
                        }
                    });
                });
            }
        }
    }
}

impl App {
    pub fn book_list_title(&mut self) {
        let mut new_list: Vec<Book> = self.book_shelf.get_books().collect();
        new_list.sort_by(|s0, s1| s0.title.cmp(&s1.title));
        self.book_list = new_list.into();
    }
}

impl App {
    fn handle(&mut self, event: AppEvent) {
        match event {
            AppEvent::SwitchState(state) => self.switch_states(state),
            AppEvent::None => {}
        }
    }
    fn switch_states(&mut self, state: AppState) {
        if self.state == state {
            return;
        }
        match state {
            AppState::BookManger => {
                self.input_faild = String::default();
            }
            _ => {}
        }
        self.state = state;
    }
}


#[derive(Default)]
pub enum AppEvent {
    #[default]
    None,
    SwitchState(AppState),
}


