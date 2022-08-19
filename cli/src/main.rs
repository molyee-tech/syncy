mod error;
mod args;
mod client;
mod hub;
mod network;
mod storage;

use std::process::ExitCode;

pub use error::{Error, Result};

use args::{Cmd, Opts};
use clap::Parser;
use colored::*;

#[cfg(feature = "fast-alloc")]
use mimalloc_rust::GlobalMiMalloc as Alloc;

#[cfg(feature = "fast-alloc")]
#[global_allocator]
static GLOBAL: GlobalAlloc = Alloc;

fn main() -> ExitCode {
    let opts = Opts::parse();
    env_logger::init();
    if let Err(e) = process(opts) {
        eprintln!("{}: {}", "Error".bright_red().bold(), &e);
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}

fn process(opts: Opts) -> Result<()> {
    match opts.command {
        Cmd::Client(c) => client::handle(c),
        Cmd::Hub(h) => hub::handle(h),
        Cmd::Network(h) => network::handle(h),
        Cmd::Storage(s) => storage::handle(s),
    }
}
