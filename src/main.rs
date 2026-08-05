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

    let root = repo_root()?;

    match opts.subcmd {
        Subcommand::Check => check(&root, opts, cfg),
        Subcommand::Clean => clean(&root, opts, cfg),
        Subcommand::Review => review(&root, opts, cfg),
    }
}

fn repo_root() -> Result<std::path::PathBuf> {
    let mut cmd = std::process::Command::new("git");
    cmd.args(["rev-parse", "--show-toplevel"]);

    let out = cmd.output()?;
    let str = std::str::from_utf8(&out.stdout)?;
    let str = str.trim_end();

    Ok(str.to_owned().into())
}
