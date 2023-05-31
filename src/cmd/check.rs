use eyre::{eyre, Result};
use owo_colors::OwoColorize;

use std::{
    fs::{self, create_dir_all, remove_dir_all, File},
    io::{stderr, BufRead, Write},
    process::exit,
};

use crate::{
    cfg::Config,
    cli::Opts,
    cmd::run::nix_check,
    proto::{TestOutput, TestResult},
};

pub fn check(opts: Opts, cfg: Option<Config>) -> Result<()> {
    let output = nix_check(opts, cfg)?;
    let success = output.status.success();

    for line in output.stderr.lines() {
        let line = line?;
        let Some(line) = line.strip_prefix("trace: namaka=") else {
                continue;
            };

        let output = serde_json::from_str::<TestOutput>(line)?;

        let pending = output.dir.join("_snapshots").join(".pending");
        let _ = remove_dir_all(&pending);
        create_dir_all(&pending)?;
        fs::write(pending.join(".gitignore"), "*")?;

        let total = output.results.len();
        let mut failures = 0;
        for (name, res) in output.results {
            let new = pending.join(&name);
            match res {
                TestResult::Success(_) => {
                    println!("{} {name}", "✔".green());
                }

                TestResult::Failure { snapshot, old } => {
                    failures += 1;
                    println!("{} {name}", if old { "✘" } else { "🞥" }.red());
                    snapshot.to_writer(File::create(new)?)?;
                }
            }
        }

        if failures == 0 {
            if success {
                eprintln!("All {total} tests succeeded");
                return Ok(());
            } else {
                break;
            }
        } else {
            eprintln!("{failures} out of {total} tests failed");
            eprintln!("run `namaka review` to review the pending snapshots");
            exit(1);
        }
    }

    stderr().write_all(&output.stderr)?;
    Err(eyre!("unknown error"))
}
