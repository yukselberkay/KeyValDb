/**
 * main.rs
 * front end code
 */

mod args;

use crate::args::Subcommands;
use libkvdb::KeyValueDB;

use std::path::{Path};

fn main() {
    let args = args::arg_parser();

    let fname = args.path;
    let db_path = Path::new(&fname);

    // opens the database file at path
    let mut store = KeyValueDB::open(db_path).expect("unable to open database file");
    // loads the offsets of any pre-existing data into an in-memory index.
    store.load().expect("unable to load data from database");

    match &args.command {
        Some(Subcommands::Get { key }) => {
            println!("Get {}", key);
            let result = store.get(key.as_bytes()).unwrap().unwrap();
            println!("{:?}", result);
        },
        Some(Subcommands::Insert { key, value }) => {
            println!("Insert {} -> {}", key, value);
            store.insert(key.as_bytes(), value.as_bytes()).unwrap();
        },
        Some(Subcommands::Delete { key }) => {
            println!("Delete -> {}", key);
        },
        Some(Subcommands::Update { key, value }) => {
            println!("Update -> {}, {}", key, value);
        },
        Some(Subcommands::List {  }) => {
            dbg!("list subcommand supplied");
            store.list();
        }
        // prints out generated help message automatically
        None => {}
    }
}
