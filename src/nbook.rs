
use audiotags::Tag;
use serde::{Deserialize, Serialize};
use std::io::Write;
use std::{
    io::{BufReader, Read},
    path::{Path, PathBuf},
};
use std::io;
use std::io::prelude::*;
use str_overlap::Overlap;
use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct Book {
    path: PathBuf,
    title: Option<String>,
    author: Vec<String>,
    narrator: Vec<String>,
    
    duration: usize,
    
}
impl Default for Book {
    fn default() -> Self {
        Self {
            path: PathBuf::default(),
            title: None,
            author: vec![],
            narrator: vec![],
            
            duration: 0,
        }
    }
}
impl std::fmt::Display for Book {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        let mut indent = 0;
        fn aux(indent: usize, output: &mut String) {
            for i in 0..indent {
                output.push(' ');
            }
        }
        if let Some(title) = self.title.clone() {
            output.push_str("Title: ");
            output.push_str(title.as_str());
            output.push_str("\n");
            indent += 4;
        }
        if !self.author.is_empty() {
            aux(indent, &mut output);
            output.push_str("Author('s): ");
            for a in self.author.iter() {
                output.push_str(a.as_str());
                output.push_str("| ");
            }
            output.push_str("\n");
            indent += 4;
        }
        if !self.narrator.is_empty() {
            aux(indent, &mut output);
            output.push_str("Narrator('s): ");
            for a in self.narrator.iter() {
                output.push_str(a.as_str());
                output.push_str("| ");
            }
            output.push_str("\n");
        }
        write!(f, "{}", output)
    }
}
impl Book {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            ..Default::default()
        }
    }
    fn from_metadata(path: PathBuf) -> Option<Book>{
        if let Ok(tag) = Tag::default().read_from_path(path.clone()){
            let mut book = Book::new(PathBuf::from(path.parent().unwrap()));
            if let Some(title) = tag.title(){
                book.title =  Some(String::from(title));
            }
            if let Some(authors) = tag.artists(){
                book.author = authors.iter().map(|s| String::from(*s)).collect();
            } 
            Some(book)
        } else {
            None
        }

        // let title = if let
    }
    pub fn init(&mut self, config: &BookConfig){
        use audiotags::*;
        let files = get_files(self.path.as_path());
        let mut p_titles: HashSet<String> = HashSet::new();
        let mut p_author: HashSet<String>  = HashSet::new();
        
        '_book: for file in files.iter(){
            if let Some(ext) = file.extension(){
                let ext = ext.to_str().unwrap();
                match ext{
                    "m4b" | "mp3" => {
                        if let Some(b) = Book::from_metadata(file.clone()){
                            if let Some(title) = b.title.clone(){
                                p_titles.insert(title);
                            }
                            let _ = b.author.into_iter().map(|a|{p_author.insert(a)});
                        }
                    },
                    
                    // do nothing
                    "sfv" => {

                    }

                    // I should pars this....
                    "txt" => {
                        // if let Some(book) = user_assisted_pars(file.as_path()){
                        //     temp_books.push(book);
                        // }
                    },
                    // marked as useless so we skipping them!
                    "_txt" => {}

                    // do nothing 
                    "torrent" => {

                    },
                    // maybe use this to order chapters 
                    "cue" => {

                    },
                    // this is the gold slandered out of the data collection proses so we are breaking this loop if we get viable data 
                    "nfo" => {
                        if let Some(b) = nfo_pars(file.as_path()){
                            *self = b;
                            return;
                        
                        }
                    },

                    // one day I will get images working 
                    "jpg" =>{},

                    _ => {
                        println!("ext:{} | file: {:?}", ext, file.as_path());
                    }
                }
            } 
        }
        if config.user_directed{

        }
        
    }
    fn _merge(mut input: Vec<Book>) -> Option<Book>{
        let mut books = input.iter_mut();
        if let Some(mut pbook) = books.next(){
            for cbook in books{
                if pbook.title.is_some() && cbook.title.is_some(){
                    let ct = cbook.title.clone().unwrap();
                    let pt = pbook.title.clone().unwrap(); 
                    let overlap = pt.overlap_end(ct.as_str());
                    println!("pt:{}|\nct:{}|\not:{}|",pt, ct, overlap);
                    pbook.title = Some(String::from(overlap));
                }
                
            }
            {
                println!("_____");
                let mut line = String::new();
                std::io::stdin().read_line(&mut line);
            }
            // if let Some(b) = pbook.title.clone(){
            //     println!("{}", b);
            // }
        } 

        None
    }
    fn merge(mut input: Vec<Book>) -> Option<Book>{
        let mut p_titles = HashSet::new();
        let mut books = input.iter_mut();
        for book in books{
            if let Some(title) = book.title.clone(){
                p_titles.insert(title);
            }
        }
        let mut p_titles: Vec<String> = p_titles.into_iter().collect();
        



        unimplemented!()
    }
    pub fn _init(&mut self) {
        use audiotags::*;
        let files = get_files(self.path.as_path());
        //exposing the iter
        let mut files_iter = files.iter();
        //using the iter to init some things
        if let Some(first) = files_iter.next() {
            if let Ok(tag) = Tag::default().read_from_path(first) {
                self.title = if let Some(s) = tag.title() {
                    Some(String::from(s))
                } else {
                    None
                };
                if let Some(e) = tag.artists() {
                    self.author = e.iter().map(|s| String::from(*s)).collect();
                }
                if let Some(os) = first.extension() {
                    if let Some(s) = os.to_str() {
                       
                    }
                }
            }
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BookConfig{
    user_directed: bool
}
impl Default for BookConfig{
    fn default() -> Self {
        Self{
            user_directed: true
        }
    }
}
fn remove_prefixed_spaces(input: &str) -> String{
    let mut trigger = true;
    let mut output = String::new();
    for c in input.chars(){
        // print!("{}|", c);
        if trigger{
            if !(c.is_whitespace()||  c == ':'){
                output.push(c);
                trigger = false;
            }
        } else {
            output.push(c);
        }
    }
    // println!("");
    output
}
fn nfo_pars(path: &Path) -> Option<Book>{
    fn aux(s: &String, f: &str) -> Option<String>{
        if let Some(id) = s.find(f){
            let (_, tail) = s.split_at(id + f.len() - 1);
            
            Some(remove_prefixed_spaces(tail))
        } else {
            None
        }
    } 
    let file = std::fs::File::open(path).expect("failed to open file");
    let buff = std::io::BufReader::new(file);
    let mut book = Book::new(PathBuf::from(path.parent().unwrap()));
    //I know I am checking every line for every condition (aka I know this slow but don't care right now)
    for line in buff.lines(){
        if let Ok(line) = line{
            let title = aux(&line, "Title:");
            if title.is_some(){
                book.title = title;
            }
            

            if let Some(author) = aux(&line, "Author:"){
                book.author.push(author);
            }
            if let Some(narrator) = aux(&line, "Read By:"){
                // println!("{}", narrator);
                book.narrator.push(narrator);
            }
            // println!("{}\n-", line);
        }
    }
    // println!("{}", book);
    Some(book)

}
fn user_assisted_pars(path: &Path) -> Option<Book>{
    fn aux() -> String{
        let mut line = String::new();
        std::io::stdin().read_line(&mut line);
        line.to_lowercase()
    }
    fn ms(data: &mut Vec<String>){
        
    }
    let mut b = Book::new(PathBuf::from(path));
    let text =  match std::fs::read_to_string(path){
        Ok(t) => t,
        Err(_) => return  None,
    };
    
    'io: loop{
        println!("[User Assisted Pars]");
        println!("for:{}", path.to_str().unwrap());
        let line = aux();
        let s = line.trim();
        match s{
            "q" | "quit" => {
                break 'io;
            },
            "pb" | "print book" => {
                println!("{}",b);
            },
            "pt" | "print text" => {
                println!("{}", text);
            }
            "ms" | "manual set" => {
                println!("[Manual Set Mod]\n");
                'ms: loop{
                    println!("Please select the data your are setting");
                    let m = aux();
                    println!("set to:");
                    match m.trim(){
                        "a" | "author" => {
                            let mut temp = vec![];
                            let mut ui = aux();
                            while ui.trim() != "" {
                                temp.push(String::from(ui.trim()));
                                ui = aux();
                            }
                            println!("are you sure? (y/N)");
                            ui = aux();
                            match ui.trim() {
                                "y" | "yes" => {
                                    b.author.append(&mut temp);
                                    
                                },
                                "n" | "no" => {
                                    
                                }
                                _ => {
                                    continue;
                                }
                            }
                            break 'ms;
                        },
                        "n" | "narrator" => {
                            let mut temp = vec![];
                            let mut ui = aux();
                            while ui.trim() != "" {
                                temp.push(String::from(ui.trim()));
                                ui = aux();
                            }
                            println!("are you sure? (y/N)");
                            ui = aux();
                            match ui.trim() {
                                "y" | "yes" => {
                                    b.narrator.append(&mut temp);
                                    
                                },
                                "n" | "no" => {
                                    
                                }
                                _ => {
                                    continue;
                                }
                            }
                            break 'ms;
                        },

                        "q" | "quit" => {
                            break 'ms;
                        }
                        _ => {

                        }
                    }
                }
            },
            "mau" | "mark as useless" => {
                println!("are you sure you want to mark this .txt file as useless?(y/N)");
                match aux().trim(){
                    "y" | "yes" => {
                        let mut n_path = PathBuf::from(path);
                        n_path.set_extension("_txt");
                        std::fs::rename(path, n_path.as_path());
                        println!("moving on");
                        break 'io;

                    },
                    _ => {}
                }
            },
            _ => {
                
            },
        };
    }
    None
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BookShelf {
    path: PathBuf,
    shelves: Vec<PathBuf>,
    books: Vec<Book>,
    ordering: Vec<usize>,
    ordering_type: (Ordering, OrderingLocation),
    book_config: BookConfig,
}
impl Default for BookShelf {
    fn default() -> Self {
        Self {
            path: PathBuf::default(),
            shelves: vec![],
            ordering_type: (Ordering::default(), OrderingLocation::default()),
            books: vec![],
            ordering: vec![],
            book_config: BookConfig::default(),
        }
    }
}
impl BookShelf {
    pub fn new(ordering_style: Ordering, ordering_location: OrderingLocation) -> Self {
        Self {
            ordering_type: (ordering_style, ordering_location),
            ..Default::default()
        }
    }
    pub fn add_shelve(&mut self, path: PathBuf) {
        self.shelves.push(path)
    }
    pub fn init(&mut self) {
        let dirs = {
            let mut temp = vec![];
            for shelf in self.shelves.iter() {
                temp.append(&mut get_dir(shelf.as_path()))
            }
            temp
        };
        self.books = dirs.iter().map(|p| Book::new(p.clone())).collect();
        for book in self.books.iter_mut() {
            book.isavnit(&self.book_config);
        }
        self.ordering = self.get_order();
    }
    fn get_data_to_order(&self) -> Vec<(usize, String)> {
        use OrderingLocation::*;
        let mut data = Vec::with_capacity(self.books.len());
        for (i, book) in self.books.iter().enumerate() {
            match self.ordering_type.1 {
                Title => {
                    if let Some(title) = book.title.clone() {
                        data.push((i, title))
                    }
                }
                Author => {
                    if let Some(author) = book.author.first() {
                        data.push((i, author.clone()))
                    }
                }
                Narrator => {
                    if let Some(narrator) = book.narrator.first() {
                        data.push((i, narrator.clone()))
                    }
                }
            };
        }
        data
    }
    pub fn set_order_style(&mut self, new: Ordering) {
        self.ordering_type.0 = new;
    }
    pub fn set_order_loacation(&mut self, new: OrderingLocation) {
        self.ordering_type.1 = new;
    }
    pub fn update_order(&mut self) {
        self.ordering = self.get_order();
    }
    fn get_order(&self) -> Vec<usize> {
        let sorter = self.ordering_type.0.get_func();
        let mut data = self.get_data_to_order();
        data.sort_by(|a, b| sorter(a, b));
        data.iter().map(|(i, _)| *i).collect()
    }
    pub fn get_books(&self) -> Vec<&Book> {
        self.ordering.iter().map(|i| &self.books[*i]).collect()
    }
    pub fn save(&mut self, path: &Path) {
        if !(path == PathBuf::default().as_path()) {
            self.path = PathBuf::from(path);
        }
        let mut s = serde_json::to_string(self).expect("failed to serialize");
        std::fs::write(self.path.as_path(), s).expect("failed to write to file");
    }
    pub fn load(path: &Path) -> Self {
        let data = std::fs::read(path).expect("failed to read in data");
        serde_json::from_slice(&data).expect("failed to deserialize")
    }
    pub fn user_fix(&mut self){
        for (i, book) in self.books.iter_mut().enumerate(){
            println!("#{}\n{}\n---", i, book);
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum Ordering {
    Alphabetic,
}

#[derive(Serialize, Deserialize, Debug)]
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
impl Default for Ordering {
    fn default() -> Self {
        Self::Alphabetic
    }
}

impl Ordering {
    pub fn get_func(
        &self,
    ) -> Box<dyn Fn(&(usize, String), &(usize, String)) -> std::cmp::Ordering> {
        match *self {
            Self::Alphabetic => Box::new(|a, b| a.1.cmp(&b.1)),
        }
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
