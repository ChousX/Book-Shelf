use chrono::Duration;
use egui_extras::RetainedImage;
use image::open;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[derive(Default)]
pub struct Book {
    pub title: String,
    pub authour: Option<String>,
    pub narrator: Option<String>,
    pub path: Option<PathBuf>,
    pub description: Option<String>,
    pub series: Option<(String, u8)>,
    pub image_path: Option<PathBuf>,
}

