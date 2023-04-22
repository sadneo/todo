use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::{fmt, fs};

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

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    init_file()?;

    match &args.command {
        Commands::Create { name } => {
            if !get_list_map()?.contains_key(name) {
                let list = List::new(name);
                set_list(name, &list)?;
            }
        }
        Commands::Destroy { name } => {
            let mut list_map = get_list_map()?;
            list_map.remove(name);
            set_list_map(&list_map)?;
        }
        Commands::Print { name } => {
            let list = get_list(name)?;
            println!("{}", list);
        }
        Commands::List => {
            let lists = get_list_map()?
                .keys()
                .fold(String::new(), |acc, str| acc + "\n" + str.as_str());
            println!("{}", lists);
        }
        Commands::Add { name, text } => {
            let mut list = get_list(name)?;
            list.push_item(Item::new(text));
            set_list(name, &list)?;
        }
        Commands::Remove { name, index } => {
            let mut list = get_list(name)?;
            list.remove_item(*index);
            set_list(name, &list)?;
        }
        Commands::Toggle { name, index } => {
            let mut list = get_list(name)?;
            list.toggle_item(*index);
            set_list(name, &list)?;
        }
    }
    Ok(())
}

fn get_list(name: &String) -> anyhow::Result<List> {
    let list_map = get_list_map()?;
    let list = list_map
        .get(name)
        .ok_or(anyhow::Error::msg("Expected there to be a list"))?;
    let list = list.clone();
    Ok(list)
}
fn set_list(name: &String, list: &List) -> anyhow::Result<()> {
    let mut list_map = get_list_map()?;
    list_map.insert(name.to_owned(), list.to_owned());
    set_list_map(&list_map)?;
    Ok(())
}

fn get_list_map() -> anyhow::Result<HashMap<String, List>> {
    let bytes = fs::read(todo_file())?;
    let list_map: HashMap<String, List> = ron::de::from_bytes(&bytes)?;
    Ok(list_map)
}
fn set_list_map(list_map: &HashMap<String, List>) -> anyhow::Result<()> {
    let contents = ron::ser::to_string(list_map)?;
    fs::write(todo_file(), contents)?;
    Ok(())
}

fn init_file() -> anyhow::Result<()> {
    if todo_file().exists() {
        return Ok(());
    }
    let list_map: HashMap<String, List> = HashMap::new();
    let contents = ron::ser::to_string(&list_map)?;
    fs::write(todo_file(), contents)?;
    Ok(())
}
fn todo_file() -> PathBuf {
    let mut dir = home::home_dir().expect("Couldn't find your home directory");
    dir.push(".mytodo");
    println!("{:?}", dir);
    dir
}
