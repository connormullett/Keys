use std::{io, path::PathBuf, str};

use db::Store;
use serde::Deserialize;
use structopt::StructOpt;
use utils::{find_db_path_or_default, read_file_to_string, read_toml};

mod db;
mod utils;

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
    pub db_path: Option<PathBuf>,
    #[structopt(long)]
    pub port: Option<u16>,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub db_path: PathBuf,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            db_path: find_db_path_or_default(),
            port: Default::default(),
        }
    }
}

impl CliOptions {
    pub fn build_config(&self) -> Result<Config, io::Error> {
        let mut config: Config = match &self.config {
            Some(config_file) => {
                let toml = read_file_to_string(&PathBuf::from(&config_file))?;
                read_toml(&toml)?
            }
            None => todo!(),
        };

        config.port = self.port.unwrap_or(5000);

        config.db_path = self.db_path.clone().unwrap_or_default();

        Ok(config)
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
                        let value = str::from_utf8(&value).expect("fixme");
                        println!("{value}");
                    }
                    None => println!("NULL"),
                }
            }
            Command::Put { key, value } => match db.put(key, value) {
                Ok(_) => println!("Success"),
                Err(e) => println!("Put request failed :: {}", e.to_string()),
            },
            Command::Delete { key } => match db.delete(key) {
                Ok(_) => println!("Success"),
                Err(e) => println!("Delete request failed :: {}", e.to_string()),
            },
        }
    }
}

fn main() {
    let Cli { options, command } = Cli::from_args();

    let config = options
        .build_config()
        .expect("FIXME: Config parsing failed");

    let db = db::Store::open_default(config.db_path.clone()).expect("FIXME: db failed to open");

    command.run(db)
}
