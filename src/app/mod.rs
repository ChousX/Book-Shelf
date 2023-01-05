mod app_state;
mod components;
mod view;

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    rc::Rc,
    str::FromStr,
};

use crate::run;
use crate::*;
pub use app_state::AppState;
use chrono::Duration;
pub use components::*;
use poll_promise::Promise;
pub use view::View;

use eframe::egui;
use egui_extras::RetainedImage;

use self::components::library;

#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)]
pub struct App {
    state: AppState,
    book_shelf: BookShelf,
    book_list: Vec<Book>,
    options: Options,
    book_manager: BookManger,
    #[serde(skip)]
    images: HashMap<PathBuf, RetainedImage>,
    #[serde(skip)]
    p_adding_books: Option<Promise<Books>>,
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
            
            images,
            p_adding_books: None,
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        //update
        let thinking = {
            let mut output = false;
            if let Some(promis) = &mut self.p_adding_books {
                if let Some(result) = promis.ready_mut() {
                    self.book_shelf.add_books(result.clone());
                    self.book_list_title();
                } else {
                    output = true;
                }
            }
            output
        };
        //gui
        self.handle(top_bar(self, ctx, thinking));
        self.handle(self.options.show(ctx));

        match self.state {
            AppState::Library => {
                self.handle(library(self, ctx, 100.0, 100.0));
            }
            AppState::Preferences => {}
            AppState::BookManger => {
                let e = self.book_manager.show(ctx);
                self.handle(e);
            }
        }
    }

    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self)
    }
}

impl App {
    pub fn book_list_title(&mut self) {
        let mut new_list: Vec<Book> = self.book_shelf.get_books().collect();
        new_list.sort_by(|s0, s1| s0.title.cmp(&s1.title));
        self.book_list = new_list.into();
    }

    pub fn from_save(cc: &eframe::CreationContext<'_>) -> Self {
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        Default::default()
    }
}

impl App {
    fn handle(&mut self, event: AppEvent) {
        match event {
            AppEvent::SwitchState(state) => self.switch_states(state),
            AppEvent::ToggleOption => self.options.visibility.toggle(),
            AppEvent::AddBooks(path) => {
                let p = poll_promise::Promise::spawn_thread("getting books", move || {
                    let path = PathBuf::from(path);
                    run(&path)
                });
                self.p_adding_books = Some(p);
            }
            AppEvent::AddImages(images) => {
                for image in images {
                    self.add_image(image);
                }
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

    fn add_image(&mut self, path: PathBuf) {
        if let Ok(bytes) = std::fs::read(&path) {
            if let Ok(image) = RetainedImage::from_image_bytes(path.to_string_lossy(), &bytes) {
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
    AddBooks(String),
    AddImages(Vec<PathBuf>),
}
