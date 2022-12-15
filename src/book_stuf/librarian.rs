
use lofty::{read_from_path, Probe};
use nfo::Nfo;
use std::cmp::Ordering;
use std::convert::TryFrom;

use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

/// Will make a vector containing book paths
pub fn get_all(root: impl Into<&Path>) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let root = root.into();
    let mut queue = {
        let dirs = match fs::read_dir(root) {
            Ok(root) => root,
            Err(_) => {
                warn!("error in reading in root files");
                continue;
            }
        };
        let mut queue = std::collections::VecDeque::new();
        for p in dirs {
            if let Ok(entry) = p {
                queue.push_back(entry.path());
            }
        }
        queue
    };

    while let Some(path) = queue.pop_front() {
        if path.is_dir() {
            if let Some(mut dirs) = dir_containing_dir(&path) {
                for dir in dirs {
                    queue.push_back(dir);
                }
            } else {
                out.push(path);
            }
        }
    }
    out
}

pub fn run(root: impl Into<&Path>) -> Books{
    let mut output = Vec::new();
    
    //so in therory all the dirs have no sub dirs
    for path in get_all(root).into_iter() {
        let mut b_title = None;
        let mut b_authour = None;
        let mut b_narrator = None;
        let mut b_path = None;
        let mut b_description = None;
        let mut b_duration = None;
        let mut b_series = None;
        let mut b_image_path = None;
        let mut b_image = None;
        if path.is_dir() {
            let dir = match fs::read_dir(path) {
                Ok(dir) => dir,
                Err(_err) => continue,
            };
            //Heaps are neet and fits our needs here!
            //sorta Heaps work as Priorty queues so not quite its ideal use
            let mut files = std::collections::BinaryHeap::new();
            for file in dir {
                if let Ok(file) = file {
                    if let Ok(ext) = Extention::try_from(&file) {
                        files.push(OrdHelper(ext, file.path()));
                    }
                }
            }
            let mut book_found = None;
            for OrdHelper(ext, path) in files.drain() {
                match ext {
                    Extention::Nfo => {
                        if let Some(info) = Nfo::new(path.clone()) {
                            //we got duh data

                            title = 
                        }
                    }
                    Extention::Cue => {
                        // only really can grab the title

                        
                    }
                    Extention::M4b => {
                        // will need to look at meta data
                        
                    }
                    Extention::Mp3 => {
                        // if we do not have the title by this point we will have to pare the file name...
                        
                    }
                    Extention::Jpg => {
                        // far futer for gui
                    }
                }
            }
        } else {
            //examin meta data
        }
    }
}

fn dir_containing_dir(dir: &PathBuf) -> Option<Vec<PathBuf>> {
    debug_assert!(dir.is_dir());

    let mut out = Vec::new();

    let dirs = match fs::read_dir(dir) {
        Ok(root) => root,
        Err(_) => {
            info!("");
            return None;
        }
    };

    for path in dirs {
        let path = match path {
            Ok(path) => path,
            Err(_) => {
                warn!("path entry failed to unwrap");
                continue;
            }
        };
        if path.path().is_dir() {
            out.push(path.path())
        }
    }

    if out.is_empty() {
        None
    } else {
        Some(out)
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum Extention {
    Jpg = 5,
    Png = 4,
    Nfo = 3,
    Cue = 2,
    M4b = 1,
    Mp3 = 0,
}

impl TryFrom<&DirEntry> for Extention {
    type Error = ();
    fn try_from(value: &DirEntry) -> Result<Self, Self::Error> {
        if let Some(value) = value.path().extension() {
            if let Some(value) = value.to_str() {
                info!("{}", value);
                use Extention::*;
                return Ok(match value {
                    "nfo" => Nfo,
                    "cue" => Cue,
                    "m4b" => M4b,
                    "mp3" => Mp3,
                    "jpg" => Jpg,
                    "png" => Png,

                    e => {
                        warn!("unsaported extention |.{}|", e);
                        return Err(());
                    }
                });
            }
        }
        Err(())
    }
}

#[derive(PartialEq, Eq)]
struct OrdHelper(Extention, PathBuf);
impl PartialOrd for OrdHelper {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.0.cmp(&other.0))
    }
}

impl Ord for OrdHelper {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl Default for Librarian {
    fn default() -> Self {
        Self {
            roots: Vec::default(),
        }
    }
}