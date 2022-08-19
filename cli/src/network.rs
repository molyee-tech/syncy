use clap::Parser;
use crate::error::Result;
use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub command: Cmd,
}

#[derive(Parser, Debug)]
pub enum Cmd {
    Lookup(LookupOpts)
}

#[derive(Parser, Debug)]
pub struct LookupOpts {
    #[clap(help = "Filter searching elements")]
    pub filter: Vec<String>, // TODO
    #[clap(parse(from_str), help = "Name or link to a specific service or node")]
    pub name: String,
}

pub fn handle(conf: Config, opts: Opts) -> Result<()> {
    Err("Unimplemented yet".into())
}