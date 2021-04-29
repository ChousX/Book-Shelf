use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub archives: Vec<PathBuf>,
    pub user_directed: bool,
    pub ordering_by: OrderingMethod,
    pub ordering_location: OrderingLocation,
    pub load: Option<PathBuf>,
    pub save: Option<PathBuf>,
}
impl Default for Config {
    fn default() -> Self {
        Self {
            archives: vec![],
            user_directed: true,
            ordering_by: OrderingMethod::default(),
            ordering_location: OrderingLocation::default(),
            load: None,
            save: None,
        }
    }
}
impl Config{
    pub fn get_possible_file_location(&self) -> Vec<PathBuf>{
        let mut output = vec![];
        for path in self.archives.iter(){
            output.append(&mut get_dir(path.as_path()));
        }
        output
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderingMethod {
    Alphabetic,
}
impl Default for OrderingMethod {
    fn default() -> Self {
        Self::Alphabetic
    }
}
impl OrderingMethod {
    pub fn get_str_func(
        &self,
    ) -> Box<dyn Fn(&(usize, String), &(usize, String)) -> std::cmp::Ordering> {
        match *self {
            Self::Alphabetic => Box::new(|a, b| a.1.cmp(&b.1)),
        }
    }
    //need to do
    pub fn get_num_func(
        &self,
    ) -> Box<dyn Fn(&(usize, String), &(usize, String)) -> std::cmp::Ordering> {
        match *self {
            Self::Alphabetic => Box::new(|a, b| a.1.cmp(&b.1)),
        }
    }
}
#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum OrderingLocation {
    Title,
    Author,
    Narrator,
}
impl Default for OrderingLocation {
    fn default() -> Self {
        Self::Title
    }
}
fn get_dir(root: &Path) -> Vec<PathBuf> {
    fn aux(root: &Path, output: &mut Vec<PathBuf>) -> std::io::Result<()> {
        for entry in std::fs::read_dir(root)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                aux(path.as_path(), output);
                output.push(path);
            }
        }
        Ok(())
    }
    let mut output = vec![];
    match aux(root, &mut output) {
        _ => {}
    }
    output
}