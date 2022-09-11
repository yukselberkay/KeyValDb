use clap::{Parser, Subcommand};

// get <key>
// insert <key> <value>
// delete <key>
// update <key> <value>
/**
 * usage: 
 * pmanager get --key=<key>
 * pmanager insert --key=<key> --value=<value>
 * pmanager delete --key=<key>
 * pmanager update --key=<key> --value=<value>
 *  */ 

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
pub struct Cli {
    /// Db path
    #[clap(short, long)]
    pub path: String,   // no Option means this arg is required.
    #[clap(subcommand)]
    pub command: Option<Subcommands>,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Get value by key from database
    Get {
        #[clap(short, long, action)]
        key: String,
    },
    /// Insert a key value pair to database
    Insert {
        #[clap(short, long, action)]
        key: String,
        #[clap(short, long, action)]
        value: String
    },
    /// Delete a key value pair from database
    Delete {
        #[clap(short, long, action)]
        key: String,
    },
    /// Update a record from database
    Update {
        #[clap(short, long, action)]
        key: String,
        #[clap(short, long, action)]
        value: String
    },
    /// List every entry from the databse
    List {

    }
}

pub fn arg_parser() -> Cli{
    let args = Cli::parse();
    args
}
