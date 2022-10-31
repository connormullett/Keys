use std::{io, path::PathBuf};

use structopt::StructOpt;

mod db;

#[derive(StructOpt)]
struct Cli {
    #[structopt(flatten)]
    pub options: CliOptions,
    #[structopt(subcommand)]
    pub command: Subcommand,
}

#[derive(Debug, StructOpt)]
struct CliOptions {
    #[structopt(long)]
    pub config: Option<String>,
    #[structopt(long)]
    pub db_path: Option<String>,
    #[structopt(long)]
    pub port: u16,
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

pub enum Subcommand {
    Get,
    Put,
    Delete,
}

impl Subcommand {
    pub fn run(&self, config: Config) {}
}

fn main() {
    let Cli { options, command } = Cli::from_args();

    let config = options
        .build_config()
        .expect("FIXME: Config parsing failed");

    command.run(config);
}
