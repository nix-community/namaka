use clap::Parser;

use std::{ffi::OsString, path::PathBuf};

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
}

#[derive(clap::Subcommand)]
pub enum Subcommand {
    /// Wrapper around `nix flake check` to prepare snapshots for failed tests
    #[command(visible_alias = "c")]
    Check {
        /// Path to the Nix flake
        dir: Option<PathBuf>,
    },
    /// Review pending snapshots and selectively accept or reject them
    #[command(visible_alias = "r")]
    Review {
        /// Path to the Nix flake
        dir: Option<PathBuf>,
    },
}
