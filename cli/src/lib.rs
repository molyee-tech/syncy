mod error;
mod config;
mod client;
mod hub;
mod network;
mod storage;

use clap::{AppSettings, Parser};
use config::Config;
use error::Result;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Parser, Debug)]
#[clap(name = "syn", author, version = formatted_version(), about, long_about=None)]
#[clap(global_settings(&[
    AppSettings::ArgsNegateSubcommands,
    AppSettings::GlobalVersion,
    AppSettings::DeriveDisplayOrder
]))]
pub struct Opts {
    #[clap(flatten)]
    pub root: RootOpts,
    #[clap(subcommand)]
    pub command: Cmd,
}

#[derive(Parser, Debug)]
pub struct RootOpts {
    #[clap(value_name = "NAME", long, short, help = "Preconfigured network profile name")]
    pub profile: Option<String>,
    #[clap(value_name = "FILE", long, short, help = "Path to custom configuration file (overlaps 'profile' option)")]
    pub config: Option<PathBuf>,
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
    let config = if let Some(path) = root.config {
        //fs::read_to_string(path)?.parse()?
        todo!()
    } else if let Some(name) = root.profile {
        /*let mut s = env::var_os("SYNCY_PROFILES_DIR");
        s.push_str(&name);
        fs::read_to_string(Path::new(s))?.parse()?*/
        todo!()
    } else {
        Config::default()
    };
    Ok(config)
}

fn formatted_version() -> &'static str {
    concat!("v", env!("CARGO_PKG_VERSION"), " ", env!("GIT_HASH"))
}
