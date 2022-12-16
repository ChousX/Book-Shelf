mod app_state;
mod components;
mod view;

use std::{collections::HashMap, path::{Path, PathBuf}};

use crate::*;
pub use app_state::AppState;
use chrono::Duration;
pub use components::*;
pub use view::View;

use eframe::{egui};
use egui_extras::RetainedImage;

use self::components::library;

pub struct App {
    state: AppState,
    book_shelf: BookShelf,
    book_list: Vec<Book>,
    options: Options,
    book_manager: BookManger,
    images: HashMap<PathBuf, RetainedImage>,
    durations: Container<Duration>,
}

impl Default for App {
    fn default() -> Self {
        
        let default_image =
            RetainedImage::from_image_bytes("default image", include_bytes!("no_pic.png")).unwrap();
        let mut images = HashMap::default();
        images.insert(PathBuf::from("no_pic.png"), default_image);
        Self {
            state: AppState::default(),
            book_shelf: BookShelf::default(),
            book_list: Vec::default(),
            options: Options::default(),
            book_manager: BookManger::default(),
            durations: Container::default(),
            images,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.handle(top_bar(self, ctx));
        self.handle(self.options.show(ctx));

        match self.state {
            AppState::Library => {
                self.handle(library(self, ctx));
            }
            AppState::Preferences => {}
            AppState::BookManger => {
                let e = self.book_manager.show(ctx);
                self.handle(e);
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
    pub fn test() -> Self {
        let mut book_shelf = BookShelf::default();
        for (i, (author, narrator)) in [
            ("Jax", "Jo"),
            ("Bob", "Alex"),
            ("Cam", "Coal"),
            ("Jax", "Jo"),
            ("Bob", "Alex"),
            ("Cam", "Bob"),
            ("Jim", "Jo"),
            ("Bob", "Alex"),
            ("Cam", "Coal"),
            ("Jax", "Jo"),
            ("Bob", "Alex"),
            ("Cam", "Coal"),
            ("Jax", "Jo"),
            ("Bob", "Alex"),
            ("Cam", "Bob"),
            ("Jim", "Jo"),
            ("Bob", "Alex"),
            ("Cam", "Coal"),
        ]
        .iter()
        .enumerate()
        {
            book_shelf.add(Book {
                title: format!("Title: {i}"),
                authour: Some(author.to_string()),
                narrator: Some(narrator.to_string()),
                ..Default::default()
            });
        }
        let mut out = Self {
            book_shelf,
            ..Default::default()
        };
        out.book_list_title();
        out
    }
}

impl App {
    fn handle(&mut self, event: AppEvent) {
        match event {
            AppEvent::SwitchState(state) => self.switch_states(state),
            AppEvent::ToggleOption => self.options.visibility.toggle(),
            AppEvent::AddBooks(books) => {
                self.book_shelf.add_books(books);
                self.book_list_title();
            }
            _ => {}
        }
    }

    fn switch_states(&mut self, state: AppState) {
        if self.state == state {
            return;
        }
        match state {
            AppState::BookManger => {
                self.book_manager = BookManger::default();
            }
            _ => {}
        }
        self.state = state;
    }

    fn add_image(&mut self, path: PathBuf){
        if let Ok(bytes) = std::fs::read(&path){
            if let Ok(image) = RetainedImage::from_image_bytes(path.to_string_lossy(), &bytes){
                self.images.insert(path, image);
            }
        }
    }
}

#[derive(Default)]
pub enum AppEvent {
    #[default]
    None,
    SwitchState(AppState),
    ToggleOption,
    AddBooks(Books),
    AddImages(Vec<PathBuf>),
}
