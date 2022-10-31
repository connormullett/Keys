use std::{io, path::PathBuf, str};

use db::Store;
use structopt::StructOpt;

mod db;

#[derive(StructOpt)]
struct Cli {
    #[structopt(flatten)]
    pub options: CliOptions,
    #[structopt(subcommand)]
    pub command: Command,
}

#[derive(Debug, StructOpt)]
struct CliOptions {
    #[structopt(long)]
    pub config: Option<String>,
    #[structopt(long)]
    pub db_path: Option<String>,
    #[structopt(long)]
    pub port: Option<u16>,
}

pub struct Config {
    pub db_path: PathBuf,
    pub port: u16,
}

impl CliOptions {
    pub fn build_config(&self) -> Result<Config, io::Error> {
        todo!()
    }
}

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::VersionlessSubcommands)]
pub enum Command {
    Get { key: String },
    Put { key: String, value: String },
    Delete { key: String },
}

impl Command {
    pub fn run(&self, db: Store) {
        match &self {
            Command::Get { key } => {
                let res = db.get(key).expect("fixme");
                match res {
                    Some(value) => {
                        let value = str::from_utf8(&value).expect("FIXME");
                        println!("{value}");
                    }
                    None => println!("NULL"),
                }
            }
            Command::Put { key, value } => todo!(),
            Command::Delete { key } => todo!(),
        }
    }
}

fn main() {
    let Cli { options, command } = Cli::from_args();

    let config = options
        .build_config()
        .expect("FIXME: Config parsing failed");

    let db = db::Store::open_default(config.db_path.clone()).expect("FIXME: db failed to open");
}
