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
    #[clap(help = "Get client daemon status")]
    Status(StatusOpts),
    #[clap(help = "Connect to network hub with selected profile or config file")]
    Connect(ConnectOpts),
    #[clap(help = "Display detailed information on the client")]
    Inspect(InspectOpts),
} 

#[derive(Parser, Debug)]
pub struct StatusOpts {
}

#[derive(Parser, Debug)]
pub struct ConnectOpts {
}

#[derive(Parser, Debug)]
pub struct InspectOpts {
}

pub fn handle(conf: Config, opts: Opts) -> Result<()> {
    Err("Unimplemented yet".into())
}