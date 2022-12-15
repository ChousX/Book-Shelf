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
