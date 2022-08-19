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
    #[clap(help = "Display default or specific network hub status")]
    Status(StatusOpts),
    #[clap(alias = "ls", help = "Display network hub list (based on profile or config)")]
    List(ListOpts),
    #[clap(help = "Display detailed information on the specific network hub")]
    Inspect(InspectOpts),
} 

#[derive(Parser, Debug)]
pub struct StatusOpts {
}

#[derive(Parser, Debug)]
pub struct ListOpts {
}

#[derive(Parser, Debug)]
pub struct InspectOpts {
}

pub fn handle(conf: Config, opts: Opts) -> Result<()> {
    Err("Unimplemented yet".into())
}