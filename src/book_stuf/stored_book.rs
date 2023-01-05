use super::Data;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Default, Debug, Deserialize, Serialize)]
pub struct StordBook {
    pub authour: Data,
    pub narrator: Data,
    pub path: Option<PathBuf>,
    pub description: Option<String>,

    pub duration: Data,
    pub series: Data,
    pub series_number: u8,
    pub image_path: Option<PathBuf>,
}
