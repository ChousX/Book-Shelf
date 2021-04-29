use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use str_overlap::Overlap;

use super::Config;
#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Book {
    pub path: PathBuf,
    pub title: Option<String>,
    pub authors: HashSet<String>,
    pub narrators: HashSet<String>,
    pub duration: usize,
}
impl Book {
    pub fn build(
        path: PathBuf,
        config: &Config,
        authors: &HashSet<String>,
        narrators: &HashSet<String>,
    ) -> Option<Book> {
        fn get_files(root: &Path) -> Vec<PathBuf> {
            fn aux(root: &Path, output: &mut Vec<PathBuf>) -> std::io::Result<()> {
                for entry in std::fs::read_dir(root)? {
                    let entry = entry?;
                    let path = entry.path();
                    if path.is_file() {
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
        fn nfo_pars(path: &Path) -> Option<Book> {
            fn aux(s: &String, f: &str) -> Option<String> {
                if let Some(id) = s.find(f) {
                    let (_, tail) = s.split_at(id + f.len() - 1);
                    Some(String::from(tail.trim()))
                } else {
                    None
                }
            }
            let file = std::fs::File::open(path).expect("failed to open file");
            let buff = std::io::BufReader::new(file);
            let mut book = Book {
                path: PathBuf::from(path),
                ..Default::default()
            };
            //I know I am checking every line for every condition (aka I know this slow but don't care right now)
            for line in buff.lines() {
                if let Ok(line) = line {
                    let title = aux(&line, "Title:");
                    if title.is_some() {
                        book.title = title;
                    }

                    if let Some(author) = aux(&line, "Author:") {
                        book.authors.insert(author);
                    }
                    if let Some(narrator) = aux(&line, "Read By:") {
                        // println!("{}", narrator);
                        book.narrators.insert(narrator);
                    }
                    // println!("{}\n-", line);
                }
            }
            // println!("{}", book);
            Some(book)
        }
        fn _add_combinations(collection: &mut HashSet<String>, entry: String) {
            if collection.get(&entry).is_none() {
                let mut acum = HashSet::new();
                for item in collection.iter() {
                    acum.insert(String::from(item.overlap_end(&entry)));
                }
                collection.insert(entry);
                acum.drain().map(|x| {
                    collection.insert(x);
                });
            }
        }
        fn get_iu() -> Option<String> {
            let mut line = String::new();
            match std::io::stdin().read_line(&mut line) {
                Ok(_) => {
                    let line = String::from(line.trim());
                    if line.is_empty() {
                        None
                    } else {
                        Some(line)
                    }
                }
                Err(_) => None,
            }
        }

        let files = get_files(path.as_path());
        let mut p_titles: HashSet<String> = HashSet::new();
        let mut p_author: HashSet<String> = HashSet::new();
        '_book: for file in files.iter() {
            if let Some(ext) = file.extension() {
                let ext = ext.to_str().unwrap();
                match ext {
                    "m4b" | "mp3" => {
                        if let Ok(tag) = audiotags::Tag::default().read_from_path(file) {
                            if let Some(title) = tag.title() {
                                p_titles.insert(String::from(title));
                            }
                            if let Some(Authors) = tag.artists() {
                                let _ = authors.into_iter().map(|a| {
                                    let s = String::from(a);
                                    p_author.insert(s);
                                });
                            }
                        }
                    }

                    // do nothing
                    "sfv" => {}

                    "txt" => if config.user_directed {},
                    // marked as useless so we skipping them!
                    "_txt" => {}

                    // do nothing
                    "torrent" => {}
                    // maybe use this to order chapters
                    "cue" => {}
                    // this is the gold slandered out of the data collection proses so we are breaking this loop if we get viable data
                    "nfo" => {
                        if let Some(b) = nfo_pars(file.as_path()) {
                            return Some(b);
                        } else if config.user_directed {
                        }
                    }

                    // one day I will get images working
                    "jpg" => {}

                    _ => {
                        println!("ext:{} | file: {:?}", ext, file.as_path());
                    }
                }
            }
            // fn _recursive_overlap(input: &mut HashSet<String>, element: Option<String>){
            //     if let Some(x) = element{
            //         for y in input.clone().iter(){
            //             let overlap_end = String::from(x.overlap_end(y).trim());
            //             if input.insert(overlap_end.clone()){
            //                 _recursive_overlap(input, Some(overlap_end));
            //             }

            //             let overlap_start = String::from(x.overlap_start(y).trim());
            //             if input.insert(overlap_start.clone()){
            //                 _recursive_overlap(input, Some(overlap_start));
            //             }
            //         }

            //     } else {
            //         for x in input.clone().into_iter(){
            //             _recursive_overlap(input, Some(x))
            //         }
            //     }
            // }
        }
        //asks user for book building input
        if config.user_directed {

            //splits selected title by user directed pattern
            fn user_split(input: &str) -> Option<String> {
                let mut numline = String::new();
                let mut mainline = String::new();
                println!("How would you like to split:\n'{}'\n:", input);
                let pat = if let Some(pat) = get_iu() {
                    pat
                } else {
                    String::from(" ")
                };
                let split = input.split(&pat);
                println!("---");
                for (count, e) in split.clone().enumerate() {
                    let f = format!("|{}|", e.trim());
                    mainline.push_str(&f);
                    numline.push_str(&count.to_string());
                    while mainline.len() > numline.len() {
                        numline.push(' ');
                    }
                }
                println!("{}", numline);
                println!("{}", mainline);
                if let Some(f) = get_iu() {
                    if let Some(t) = get_iu() {
                        if let Ok(f) = f.parse::<usize>() {
                            if let Ok(t) = t.parse::<usize>() {
                                let mut out = String::new();
                                split.enumerate().for_each(|(i, e)| {
                                    if i >= f && i < t + 1 {
                                        out.push_str(e.trim());
                                        out.push(' ');
                                    }
                                });
                                println!();
                                return Some(out);
                            }
                        }
                    }
                    println!();
                }
                None
            }
            // user select an entry out of a hash-set
            fn select(set: &HashSet<String>) -> Option<String> {
                fn aux(set: &HashSet<String>) {
                    //listing off all the posable book titles
                    for (i, item) in set.iter().enumerate() {
                        println!("{}|'{}'", i, item);
                    }
                    // print!("Select or enter nothing to continue");
                    // ^ for some reason this was not printing befor the get_iu funk
                }

                aux(set);
                if let Some(id) = get_iu() {
                    if let Ok(id_parsed) = id.parse::<usize>() {
                        println!();
                        return set.iter().enumerate().find_map(|(i, e)| {
                            if id_parsed == i {
                                Some(e.clone())
                            } else {
                                None
                            }
                        });
                    }
                }
                println!();
                None
            }
            //prompt user with options and display the data thats being worked on
            fn user_option(input: &str) -> Option<String> {
                let save = String::from(input);
                println!("candiot:'{}'\nuser options:\n0: Split", input);
                if let Some(input) = get_iu() {
                    match input.to_lowercase().trim() {
                        "0" | "s" | "split" => {
                            if let Some(choice) = user_split(&save) {
                                println!("Choice: '{}'", choice);
                                Some(choice)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    }
                } else {
                    None
                }
            }
            //Title
            let title = if !p_titles.is_empty() {
                println!("Title");
                if let Some(title_) = select(&p_titles) {
                    user_option(&title_)
                } else {
                    if let Some(s) = p_titles.iter().next() {
                        Some(String::from(s))
                    } else {
                        None
                    }
                }
            } else {
                None
            };
            println!("Author");
            let mut authors = HashSet::new();
            loop{
                let author_ = if !p_author.is_empty(){

                    if let Some(author_) = select(&p_author){
                        user_option(&author_)
                    } else {
                        if let Some(s) = p_author.iter().next(){
                            Some(String::from(s))
                        } else {
                            None
                        }
                    }
                } else {
                    None
                };
                if let Some(author_) =author_{
                    authors.insert(author_);
                } else {
                    break;
                }
            };
            Some(Self{title, authors, ..Default::default()})
            
        } else {
            None
        } 
    }
}
impl std::fmt::Display for Book{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut acum = String::new();
        if let Some(title) = self.title.clone(){
            acum.push_str("Title: ");
            acum.push_str(&title);
            acum.push_str("\n");
        }
        if !self.authors.is_empty(){
            acum.push_str("Author('s): ");
            let mut a_iter = self.authors.iter();
            if let Some(author) = a_iter.next(){
                acum.push_str(author);
            }
            for author in a_iter{
                acum.push_str(", ");
                acum.push_str(author);
            }
            acum.push_str("\n")
        }
        if !self.narrators.is_empty(){
            acum.push_str("Narrator('s): ");
            let mut n_iter = self.narrators.iter();
            if let Some(narrator) = n_iter.next(){
                acum.push_str(narrator);
            }
            for narrator in n_iter{
                acum.push_str(", ");
                acum.push_str(narrator);
            }
            acum.push_str("\n")
        }
        write!(f, "{}", acum)
    }
}
