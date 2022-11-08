use std::{io, path::PathBuf, str, sync::Arc};

use db::Store;
use serde::Deserialize;
use structopt::StructOpt;
use tokio::task::JoinHandle;
use utils::{
    find_default_config, init_default_data_dir, read_file_to_string, read_toml, set_sigint_handler,
};

mod db;
#[cfg(feature = "http-server")]
mod http;
mod utils;

#[derive(StructOpt)]
struct Cli {
    #[structopt(flatten)]
    pub options: CliOptions,
    #[structopt(subcommand)]
    pub command: Option<Command>,
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
    pub data_dir: PathBuf,
    pub port: u16,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            data_dir: init_default_data_dir(),
            port: 5000,
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
            None => find_default_config().unwrap_or_default(),
        };

        config.port = self.port.unwrap_or(5000);

        config.data_dir = self.db_path.clone().unwrap_or_else(init_default_data_dir);

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
    pub fn run(&self, db: Arc<Store>) {
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
                Err(e) => println!("Put request failed :: {}", e),
            },
            Command::Delete { key } => match db.delete(key) {
                Ok(_) => println!("Success"),
                Err(e) => println!("Delete request failed :: {}", e),
            },
        }
    }
}

#[allow(unused)]
async fn start(config: Arc<Config>, db: Arc<Store>) {
    let ctrlc_oneshot = set_sigint_handler();

    let mut services: Vec<JoinHandle<()>> = vec![];

    #[cfg(feature = "http-server")]
    let http_service = http::start_server(config, db).await;
    #[cfg(feature = "http-server")]
    services.push(http_service);

    ctrlc_oneshot.await.unwrap();

    for handle in services {
        handle.abort();
    }
}

#[tokio::main]
async fn main() {
    let Cli { options, command } = Cli::from_args();

    let config = Arc::new(
        options
            .build_config()
            .expect("FIXME: Config parsing failed"),
    );

    let db = Arc::new(
        db::Store::open_default(config.data_dir.clone()).expect("FIXME: db failed to open"),
    );

    // Run a command or start the server
    match command {
        Some(command) => command.run(db),
        None => start(config, db).await,
    }
}
