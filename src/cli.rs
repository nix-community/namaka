use clap::Parser;

use std::path::PathBuf;

/// Snapshot testing tool for Nix based on haumea
/// https://github.com/nix-community/namaka
#[derive(Parser)]
#[command(version, verbatim_doc_comment)]
pub struct Opts {
    #[command(subcommand)]
    pub cmd: Subcommand,
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
