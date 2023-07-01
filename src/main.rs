mod cfg;
mod cli;
mod cmd;
mod proto;

use std::env::set_current_dir;

use clap::Parser;
use eyre::Result;

use crate::{
    cli::{Opts, Subcommand},
    cmd::{check, clean, review},
};

fn main() -> Result<()> {
    let _ = color_eyre::install();

    let opts = Opts::parse();
    if let Some(dir) = &opts.dir {
        set_current_dir(dir)?;
    }

    let cfg = cfg::load()?;

    match opts.subcmd {
        Subcommand::Check => check(opts, cfg),
        Subcommand::Clean => clean(opts, cfg),
        Subcommand::Review => review(opts, cfg),
    }
}
