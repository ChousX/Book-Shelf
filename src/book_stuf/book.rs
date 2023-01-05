use std::path::PathBuf;

use chrono::Duration;
#[derive(serde::Deserialize, serde::Serialize)]

#[derive(Default, Clone)]
pub struct Book {
    pub title: String,
    pub authour: Option<String>,
    pub narrator: Option<String>,
    pub path: Option<PathBuf>,
    pub description: Option<String>,
    pub series: Option<(String, u8)>,
    pub image_path: Option<PathBuf>,
    #[serde(skip)]
    pub duration: Option<Duration>,
}
