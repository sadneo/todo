use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::{fmt, fs, path::Path};

#[derive(Serialize, Deserialize, Debug, Clone)]
struct List {
    name: String,
    items: Vec<Item>,
}

impl List {
    fn new(name: &String) -> List {
        let name = name.to_string();
        let items = vec![];
        List { name, items }
    }
    fn push_item(&mut self, item: Item) {
        self.items.push(item);
    }
    fn remove_item(&mut self, index: usize) {
        self.items.remove(index);
    }
    fn toggle_item(&mut self, index: usize) {
        let item = self.items.get_mut(index);
        if let Some(item) = item {
            item.toggle();
        }
    }
}
impl fmt::Display for List {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let b = self
            .items
            .iter()
            .fold(String::new(), |acc, x| acc + "\n" + x.to_string().as_str());
        write!(f, "{}:{}", self.name, b)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Item {
    checked: bool,
    text: String,
}

impl Item {
    fn new(text: &String) -> Item {
        let checked = false;
        let text = text.to_string();
        Item { checked, text }
    }
    fn toggle(&mut self) {
        self.checked = !self.checked;
    }
}
impl fmt::Display for Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let check = if self.checked { "X" } else { " " };
        write!(f, "[{}] {}", check, self.text)
    }
}

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug, Clone)]
enum Commands {
    Create { name: String },
    Destroy { name: String },
    Print { name: String },
    List,
    Add { name: String, text: String },
    Remove { name: String, index: usize },
    Toggle { name: String, index: usize },
}

fn main() {
    let args = Args::parse();
    init_file();

    match &args.command {
        Commands::Create { name } => {
            if list_exists(name) {
                println!("poop");
                return;
            }
            let list = List::new(name);
            set_list(name, &list);
        }
        Commands::Destroy { name } => {
            remove_list(name);
        }
        Commands::Print { name } => {
            let list = get_list(name);
            if let None = list {
                return;
            }
            let list = list.unwrap();
            println!("{}", list);
        }
        Commands::List => {
            let lists = get_lists();
            println!("{}", lists);
        }
        Commands::Add { name, text } => {
            let list = get_list(name);
            let item = Item::new(text);
            if let None = list {
                return;
            }
            let mut list = list.unwrap();
            list.push_item(item);
            set_list(name, &list);
        }
        Commands::Remove { name, index } => {
            let list = get_list(name);
            if let None = list {
                return;
            }
            let mut list = list.unwrap();
            list.remove_item(*index);
            set_list(name, &list);
        }
        Commands::Toggle { name, index } => {
            let list = get_list(name);
            if let None = list {
                return;
            }
            let mut list = list.unwrap();
            list.toggle_item(*index);
            set_list(name, &list);
        }
    }
}

fn get_list(name: &String) -> Option<List> {
    if !exists() {
        return None;
    }

    let bytes =
        fs::read("/home/aidan/.mytodo").expect("There should be a file here after running init_file()");
    let list_map: HashMap<String, List> = ron::de::from_bytes(&bytes).expect("Invalid ron notation found");
    let list = list_map.get(name)
        .expect("There isn't any list called this name")
        .clone();

    Some(list)
}
fn set_list(name: &String, list: &List) {
    if !exists() {
        return;
    }

    let bytes =
        fs::read("/home/aidan/.mytodo").expect("There should be a file here after running init_file()");
    let mut list_map: HashMap<String, List> = ron::de::from_bytes(&bytes).expect("Invalid ron notation found");
    list_map.insert(name.to_owned(), list.to_owned());

    let contents = ron::ser::to_string(&list_map).expect("Ron couldn't write for some reason");
    fs::write("/home/aidan/.mytodo", contents).expect("Yeah sure this should work");
}
fn remove_list(name: &String) {
    if !exists() {
        return;
    }

    let bytes =
        fs::read("/home/aidan/.mytodo").expect("There should be a file here after running init_file()");
    let mut list_map: HashMap<String, List> = ron::de::from_bytes(&bytes).expect("Invalid ron notation found");
    list_map.remove(name);

    let contents = ron::ser::to_string(&list_map).expect("Ron couldn't write for some reason");
    fs::write("/home/aidan/.mytodo", contents).expect("Yeah sure this should work");
}
fn get_lists() -> String {
    if !exists() {
        return String::from("<nothing>");
    }

    let bytes =
        fs::read("/home/aidan/.mytodo").expect("There should be a file here after running init_file()");
    let list_map: HashMap<String, List> = ron::de::from_bytes(&bytes).expect("Invalid ron notation found");
    list_map.keys().fold(String::new(), |acc, str| acc + "\n" + str.as_str())
}

fn init_file() {
    if exists() {
        return;
    }

    let todos: HashMap<String, List> = HashMap::new();
    let contents = ron::ser::to_string(&todos).expect("Ron couldn't write for some reason");
    std::fs::File::create("/home/aidan/mytodo").expect("Yeah sure this should work x2");
    fs::write("/home/aidan/.mytodo", contents).expect("Yeah sure this should work");
}
fn exists() -> bool {
    Path::new("/home/aidan/.mytodo").exists()
}
fn list_exists(name: &String) -> bool {
    if !exists() {
        return false;
    }

    let bytes =
        fs::read("/home/aidan/.mytodo").expect("There should be a file here after running init_file()");
    let list_map: HashMap<String, List> = ron::de::from_bytes(&bytes).expect("Invalid ron notation found");
    list_map.contains_key(name)
}

