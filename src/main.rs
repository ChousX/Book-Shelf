#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{collections::VecDeque, default, path::PathBuf};

use image::ImageBuffer;
use poll_promise::Promise;

use book_shelf::*;
use eframe::{egui, epaint::ahash::HashMap};
use egui_extras::RetainedImage;

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

#[derive(Default, Clone, Copy, PartialEq)]
pub enum AppState {
    #[default]
    Library,
    Preferences,
    Player,
    BookManger,
}
impl ToString for AppState {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Library => "Library",
            Self::Player => "Player",
            Self::Preferences => "Preferences",
            Self::BookManger => "BookManger",
        })
    }
}

#[derive(Default)]
pub enum AppEvent {
    #[default]
    None,
    SwitchState(AppState),
}

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

#[derive(Default)]
pub enum View {
    #[default]
    Shown,
    Hidden,
}

impl View {
    pub fn is_visible(&self) -> bool {
        match self {
            Self::Hidden => false,
            Self::Shown => true,
        }
    }

    pub fn toggle(&mut self) {
        let t = match self {
            Self::Hidden => Self::Shown,
            Self::Shown => Self::Hidden,
        };
        *self = t;
    }
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
