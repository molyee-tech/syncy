mod error;
mod config;
mod client;
mod hub;
mod network;
mod storage;

use clap::Parser;
use config::Config;
use error::Result;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Opts {
    #[clap(flatten)]
    pub root: RootOpts,
    #[clap(subcommand)]
    pub command: Cmd,
}

#[derive(Parser, Debug)]
pub struct RootOpts {
    #[clap(help = "Preconfigured network profile name")]
    pub profile: Option<String>,
    #[clap(long, short, help = "Path to custom configuration file (overlaps 'profile' option)")]
    pub config: Option<PathBuf>
}

#[derive(Parser, Debug)]
pub enum Cmd {
    #[clap(about = "Manage local client daemon")]
    Client(client::Opts),
    #[clap(about = "Manage network hubs")]
    Hub(hub::Opts),
    #[clap(alias = "net", about = "Manage network cluster nodes")]
    Network(network::Opts),
    #[clap(about = "Manage data storage service")]
    Storage(storage::Opts),
} 

pub fn process(opts: Opts) -> Result<()> {
    let conf = get_config(opts.root)?;
    match opts.command {
        Cmd::Client(c) => client::handle(conf, c),
        Cmd::Hub(h) => hub::handle(conf, h),
        Cmd::Network(h) => network::handle(conf, h),
        Cmd::Storage(s) => storage::handle(conf, s),
    }
}

fn get_config(root: RootOpts) -> Result<Config> {
    // TODO
    Ok(Config {})
}