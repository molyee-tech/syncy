use clap::Parser;
use crate::error::Result;
use crate::config::Config;

#[derive(Parser, Debug)]
pub struct Opts {

}

pub fn handle(conf: Config, opts: Opts) -> Result<()> {
    Err("Unimplemented yet".into())
}