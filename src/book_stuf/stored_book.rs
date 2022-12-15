use super::Data;
use chrono::Duration;
use std::path::PathBuf;

#[derive(Default)]
pub struct StordBook {
    pub authour: Data,
    pub narrator: Data,
    pub path: Option<PathBuf>,
    pub description: Option<String>,
    pub duration: Option<Duration>,
    pub series: Data,
    pub series_number: u8,
    pub image_path: Option<PathBuf>,
}