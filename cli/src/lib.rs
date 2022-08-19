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
    #[clap(help = "Path to custom configuration file (overlaps 'profile' option)")]
    pub config: Option<PathBuf>
}

#[derive(Parser, Debug)]
pub enum Cmd {
    #[clap(help = "Commands relative to local client daemon")]
    Client(client::Opts),
    #[clap(help = "Commands relative to network hub (from profile settings)")]
    Hub(hub::Opts),
    #[clap(alias = "network", help = "Commands relative to network cluster (information from hubs)")]
    Network(network::Opts),
    #[clap(help = "Commands relative to network storage nodes")]
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