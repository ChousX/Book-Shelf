use std::path::PathBuf;
use unicode_segmentation::UnicodeSegmentation;

pub fn get_title<P: Into<PathBuf>>(path: P) -> Option<String>{
    let path = path.into();
    todo!()
} 