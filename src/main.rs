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
            .fold(String::new(), |acc, x| acc + x.to_string().as_str());
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
    Add { name: String, text: String },
    Remove { name: String, index: usize },
    Toggle { name: String, index: usize },
}

fn main() {
    let args = Args::parse();
    init_file();

    match &args.command {
        Commands::Create { name } => {
            // check if list is there, if it's not,
            // then create a list, serialize it, and write it
        }
        Commands::Destroy { name } => {
            // check if the list is there, if it is,
            // then remove it from the vector, serialize it, and write it
        }
        Commands::Add { name, text } => {
            let list = get_list(name);
            let item = Item::new(text);
            if let Some(mut list) = list {
                list.push_item(item);
            }
        }
        Commands::Remove { name, index } => {
            let list = get_list(name);
            if let Some(mut list) = list {
                list.remove_item(*index);
            }
        }
        Commands::Toggle { name, index } => {
            let list = get_list(name);
            if let Some(mut list) = list {
                list.toggle_item(*index);
            }
        }
    }
}

fn get_list(name: &String) -> Option<List> {
    if !exists() {
        return None;
    }

    let bytes =
        fs::read("~/.mytodo").expect("There should be a file here after running init_file()");
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
        fs::read("~/.mytodo").expect("There should be a file here after running init_file()");
    let mut list_map: HashMap<String, List> = ron::de::from_bytes(&bytes).expect("Invalid ron notation found");
    list_map.insert(name.to_owned(), list.to_owned());

    let contents = ron::ser::to_string(&list_map).expect("Ron couldn't write for some reason");
    fs::write("~/.mytodo", contents).expect("Yeah sure this should work");
}

fn init_file() {
    if exists() {
        return;
    }

    let todos: HashMap<String, List> = HashMap::new();
    let contents = ron::ser::to_string(&todos).expect("Ron couldn't write for some reason");
    fs::write("~/.mytodo", contents).expect("Yeah sure this should work");
}
fn exists() -> bool {
    Path::new("~/.mytodo").exists()
}
