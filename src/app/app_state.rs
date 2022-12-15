#[derive(Default, Clone, Copy, PartialEq)]
pub enum AppState {
    #[default]
    Library,
    Preferences,
    BookManger,
}

impl ToString for AppState {
    fn to_string(&self) -> String {
        String::from(match self {
            Self::Library => "Library",
            Self::Preferences => "Preferences",
            Self::BookManger => "BookManger",
        })
    }
}
