use std::fmt;

#[derive(Debug)]
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

#[derive(Debug)]
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
    Create {
        name: String,
    },
    Destroy {
        name: String,
    },
    Add {
        name: String,
        text: String,
    },
    Remove {
        name: String,
        index: usize,
    },
    Toggle {
        name: String,
        index: usize,
    },
}

fn main() {
    let args = Args::parse();
    match &args.command {
        Commands::Create{ name } => {
            let list = List::new(name);
            println!("{}", list);
        },
        Commands::Destroy{ name } => destroy(name),
        Commands::Add{name, text} => {
            let list = get(name);
            let item = Item::new(text);
            if let Some(mut list) = list {
                list.push_item(item);
            }
        },
        Commands::Remove{name, index} => {
            let list = get(name);
            if let Some(mut list) = list {
                list.remove_item(*index);
            }
        },
        Commands::Toggle{name, index} => {
            let list = get(name);
            if let Some(mut list) = list {
                list.toggle_item(*index);
            }
        },
    }
}

fn create(name: &String) -> List {
    // need to add to file
    List::new(name)
}
fn destroy(name: &String) {
    // need to remove from file
}
fn get(name: &String) -> Option<List> {
    // need to get from file
    None
}