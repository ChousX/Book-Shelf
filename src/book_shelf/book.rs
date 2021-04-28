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
        let mut title = None;
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
                print!("from:");
                if let Some(f) = get_iu() {
                    print!(" to:");
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
            fn select(set: HashSet<String>) -> Option<String>{
                for (i, item) in set.iter().enumerate() {
                    println!("{}|'{}'", i, item);
                }
                println!("Select or enter nothing to continue\n'id'");
                if let Some(id) = get_iu(){
                    if let Ok(id_parsed) = id.parse::<usize>(){
                        return set.iter().enumerate().find_map(|(i, e)|{ if id_parsed == i{Some(e.clone())}else{None}});
                    }
                } 
                None
            }
            fn user_option(input: ){
                println!("candiot:'{}'\nuser options:\n0: Split\n", s_title);
            }
            //listing off all the posable book titles

            //asking the user to select an title entry. if they enter nothing or a non number or a value that is out of bands move on
            println!("Select or enter nothing to continue\n'id'");

            if let Some(id) = get_iu() {
                if let Ok(id_) = id.parse::<usize>() {
                    println!("-0");
                    if let Some(s_title) =
                    
                        p_author.iter().enumerate().find_map(
                            |(i, e)| {
                                if i == id_ {
                                    Some(e)
                                } else {
                                    None
                                }
                            },
                        )
                    {
                    println!("-1");
                        println!("candiot:'{}'\nuser options:\n0: Split\n", s_title);
                        if let Some(option) = get_iu() {
                            title = match option.to_lowercase().trim() {
                                "0" | "s" | "split" => {
                                    if let Some(choice) = user_split(s_title) {
                                        println!("Choice: '{}'", choice);
                                        Some(choice)
                                    } else {
                                        p_titles.into_iter().next()
                                    }
                                }
                                _ => p_titles.into_iter().next(),
                            };
                        }
                    }
                }
            } else {
                title = p_titles.into_iter().next();
            }
        } else {
        }

        None
    }
}
