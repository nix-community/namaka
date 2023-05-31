use eyre::{eyre, Result};
use owo_colors::OwoColorize;

use std::{
    fs::{read_dir, remove_dir_all, remove_file},
    io::{stderr, BufRead, Write},
};

use crate::{cfg::Config, cli::Opts, cmd::run::nix_eval, proto::TestOutput};

pub fn clean(opts: Opts, cfg: Option<Config>) -> Result<()> {
    let output = nix_eval(opts, cfg)?;

    for line in output.stderr.lines() {
        let line = line?;
        let Some(line) = line.strip_prefix("trace: namaka=") else {
            continue;
        };

        let mut out = stderr().lock();
        let output = serde_json::from_str::<TestOutput>(line)?;
        let snapshots = output.dir.join("_snapshots");

        for entry in read_dir(snapshots)? {
            let entry = entry?;
            let name = entry.file_name();
            let name = name
                .to_str()
                .ok_or_else(|| eyre!("invalid unicode in file name {}", name.to_string_lossy()))?;

            if !output.results.contains_key(name) {
                let path = entry.path();
                writeln!(out, "{} {}", "removing".yellow(), path.to_string_lossy())?;
                if path.is_dir() {
                    remove_dir_all(path)?;
                } else {
                    remove_file(path)?;
                }
            }
        }
    }

    Ok(())
}
