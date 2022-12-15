use std::path::PathBuf;
use chrono::Duration;
use egui_extras::RetainedImage;
use image::open;

#[derive(Default)]
pub struct Book {
    pub title: String,
    pub authour: Option<String>,
    pub narrator: Option<String>,
    pub path: Option<PathBuf>,
    pub description: Option<String>,
    pub duration: Option<Duration>,
    pub series: Option<(String, u8)>,
    pub image_path: Option<PathBuf>,
    pub image: Option<RetainedImage>,
}

impl Book {
    pub fn set_image(&mut self) -> bool {
        if let Some(path) = &self.image_path {
            let image_bytes = match open(path) {
                Ok(file) => file.to_rgb8().into_raw(),
                _ => return false,
            };
            if let Ok(image) = RetainedImage::from_image_bytes(&self.title, &image_bytes) {
                self.image = Some(image);
                true
            } else {
                false
            }
        } else {
            false
        }
    }
}