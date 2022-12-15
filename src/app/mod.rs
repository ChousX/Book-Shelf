mod app_state;
mod components;
mod view;

use crate::*;
pub use app_state::AppState;
pub use components::*;
pub use view::View;

use eframe::egui;
use egui_extras::RetainedImage;

use self::components::library;

pub struct App {
    state: AppState,
    book_shelf: BookShelf,
    book_list: Vec<Book>,
    options: Options,
    book_manager: BookManger,
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
            book_manager: BookManger::default(),
            default_image,
        };
        out.book_list_title();
        out
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
                self.book_manager.show(ctx);
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
            AppEvent::ToggleOption => self.options.visibility.toggle(),
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
}

#[derive(Default)]
pub enum AppEvent {
    #[default]
    None,
    SwitchState(AppState),
    ToggleOption,
}
