use std::{ffi::OsString, path::PathBuf};

use clap::Parser;

/// Snapshot testing for Nix based on haumea
/// https://github.com/nix-community/namaka
#[derive(Parser)]
#[command(version, verbatim_doc_comment)]
pub struct Opts {
    #[command(subcommand)]
    pub subcmd: Subcommand,

    /// Command to run instead of `nix flake check`
    ///
    /// Example: namaka check -c nix eval .#checks
    #[arg(short, long, num_args = 1 .., global = true)]
    pub cmd: Option<Vec<OsString>>,

    /// Change to this working directory
    #[arg(global = true)]
    pub dir: Option<PathBuf>,
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Wrapper around `nix flake check` to prepare snapshots for failed tests
    #[command(visible_alias = "c")]
    Check,
    /// Remove unused and pending snapshots
    #[command(visible_alias = "cl")]
    Clean,
    /// Review pending snapshots and selectively accept or reject them
    #[command(visible_alias = "r")]
    Review,
}
