use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
pub struct Opts {
    #[clap(subcommand)]
    pub commands: Command,
}

#[derive(Parser)]
pub enum Command {

}