use crate::share::*;
use crate::BookShelf;
use lofty::{read_from_path, Probe};
use nfo::Nfo;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::ffi::OsStr;
use std::fs;
use std::fs::DirEntry;
use std::path::PathBuf;

/// A builder ish styled object that will catilog files comming in
pub struct Librarian {
    roots: Vec<PathBuf>,
}

impl Librarian {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add<T: Into<PathBuf>>(&mut self, dir: T) -> &mut Self {
        self.roots.push(dir.into());
        self
    }

    /// Will make a vector containing book paths
    pub fn get_all(&self) -> Vec<PathBuf> {
        let mut out = Vec::new();
        for root in self.roots.iter() {
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
        }
        out
    }
    pub fn run(&self) {
        //, book_shelf: &mut BookShelf

        //so in therory all the dirs have no sub dirs
        for path in self.get_all().into_iter() {
            if path.is_dir() {
                let dir = match fs::read_dir(path) {
                    Ok(dir) => dir,
                    Err(_err) => continue,
                };
                let mut files = std::collections::BinaryHeap::new();
                for file in dir {
                    if let Ok(file) = file {
                        if let Ok(ext) = Extention::try_from(&file) {
                            files.push(OrdHelper(ext, file.path()));
                        }
                    }
                }

                for OrdHelper(ext, path) in files.drain() {
                    match ext {
                        Extention::Nfo => {
                            if let Some(info) = Nfo::new(path) {
                                println!("{:?}", info)
                            }
                        }
                        Extention::Cue => {}
                        Extention::M4b => {}
                        Extention::Mp3 => {}
                        Extention::Jpg => {}
                    }
                }
            } else {
                //examin meta data
            }
        }
    }
}

/*
   Dir{
       .m4b
   }

   Dir{
       .m4b
       .nfo
   }

   Dir{
       .cue
       .jpg
       .m4b
       .nfo
   }



   Dir{
       .jpg
       .jpg
       .cue
       .m4b
       .nfo
   }
*/

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
    Nfo = 4,
    Cue = 3,
    M4b = 2,
    Mp3 = 1,
    Jpg = 0,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn try_from_OsStr_Extention() {
        todo!()
    }
}
