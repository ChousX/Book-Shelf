use image::codecs::png::PngEncoder;
use image::DynamicImage;
use lofty::{read_from_path, Probe};
use nfo::Nfo;
use std::cmp::Ordering;
use std::convert::TryFrom;

use image::ImageEncoder;
use std::fs::DirEntry;
use std::fs::{self, File};
use std::io::BufWriter;
use std::path::{Path, PathBuf};

use crate::{Book, Books};

/// Will make a vector containing book paths
pub fn get_all(root: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let mut queue = {
        let dirs = match fs::read_dir(root) {
            Ok(root) => root,
            Err(_) => {
                return out;
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

pub fn run(root: &PathBuf) -> Books {
    let mut output = Vec::new();

    //so in therory all the dirs have no sub dirs
    'fp: for file_path in get_all(root).into_iter() {
        let mut title = None;
        let mut authour = None;
        let mut narrator = None;
        let mut description = None;
        // let mut duration = None;
        // let mut series = None;
        let mut image_path = None;
        // let mut image = None;
        if file_path.is_dir() {
            let dir = match fs::read_dir(file_path.clone()) {
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
            for OrdHelper(ext, path) in files.drain() {
                match ext {
                    Extention::Nfo => {
                        if let Some(info) = Nfo::new(path.clone()) {
                            title = info.general.title;
                            authour = info.general.author;
                            narrator = info.general.read_by;
                            description = info.description
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
                        if image_path.is_none() {
                            //convert the jpg in to a png
                            let target = {
                                let name = path.file_name().unwrap();
                                let name = name.to_string_lossy();
                                let name = name.strip_suffix(".jpg").unwrap();
                                let mut path = path.clone();
                                path.pop();
                                path.push(format!("{name}.png"));
                                path
                            };
                            if let Ok(file) = image::open(&path) {
                                if encode_png(&file, target.clone()) {
                                    println!("{:?}", target);
                                    image_path = Some(target);
                                }
                            }
                        }
                    }
                    Extention::Png => {
                        image_path = Some(path);
                    }
                }
                if title.is_some() {
                    output.push(Book {
                        title: title.unwrap(),
                        narrator,
                        authour,
                        description,
                        path: Some(file_path),
                        image_path,
                        ..Default::default()
                    });
                    continue 'fp;
                }
            }
        } else {
            //examin meta data
        }
    }
    output.into()
}

fn dir_containing_dir(dir: &PathBuf) -> Option<Vec<PathBuf>> {
    debug_assert!(dir.is_dir());

    let mut out = Vec::new();

    let dirs = match fs::read_dir(dir) {
        Ok(root) => root,
        Err(_) => {
            return None;
        }
    };

    for path in dirs {
        let path = match path {
            Ok(path) => path,
            Err(_) => {
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
    Png = 5,
    Jpg = 4,
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
                use Extention::*;
                return Ok(match value {
                    "nfo" => Nfo,
                    "cue" => Cue,
                    "m4b" => M4b,
                    "mp3" => Mp3,
                    "jpg" => Jpg,
                    "png" => Png,

                    _e => {
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

fn encode_png(img: &DynamicImage, name: PathBuf) -> bool {
    let file = File::create(name).unwrap();
    let ref mut buff = BufWriter::new(file);
    let encoder = PngEncoder::new(buff);
    encoder
        .write_image(img.as_bytes(), img.width(), img.height(), img.color())
        .is_ok()
}
