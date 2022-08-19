use clap::Parser;
use colored::*;
use std::process::ExitCode;
use syncy_cli::{Opts, process};

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
