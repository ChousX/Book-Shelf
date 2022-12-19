use std::path::PathBuf;

use chrono::Duration;
#[derive(Default, Clone)]
pub struct Book {
    pub title: String,
    pub authour: Option<String>,
    pub narrator: Option<String>,
    pub path: Option<PathBuf>,
    pub description: Option<String>,
    pub series: Option<(String, u8)>,
    pub image_path: Option<PathBuf>,
    pub duration: Option<Duration>,
}
