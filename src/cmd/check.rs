use std::{
    fs::{self, create_dir_all, remove_dir_all, File},
    io::{stderr, BufRead, Write},
    path::Path,
    process::exit,
};

use eyre::{eyre, Result};
use owo_colors::OwoColorize;

use crate::{
    cfg::Config,
    cli::Opts,
    cmd::run::nix_check,
    proto::{TestOutput, TestResult},
};

pub fn check(root: &Path, opts: Opts, cfg: Option<Config>) -> Result<()> {
    let output = nix_check(opts, cfg)?;
    let success = output.status.success();

    for line in output.stderr.lines() {
        let line = line?;
        let Some(line) = line.strip_prefix("trace: namaka=") else {
            continue;
        };

        let output = serde_json::from_str::<TestOutput>(line)?;

        let pending = root.join(output.dir).join("_snapshots/.pending");
        let _ = remove_dir_all(&pending);
        create_dir_all(&pending)?;
        fs::write(pending.join(".gitignore"), "*")?;

        let total = output.results.len();
        let mut failures = 0;
        let mut additions = 0;
        for (name, res) in output.results {
            let new = pending.join(&name);
            match res {
                TestResult::Success(_) => {
                    println!("{} {name}", "✔".green());
                }

                TestResult::Failure { snapshot, old } => {
                    if old {
                        failures += 1;
                        println!("{} {name}", "✘".red());
                    } else {
                        additions += 1;
                        println!("{} {name}", "🞥".blue());
                    }
                    snapshot.to_writer(File::create(new)?)?;
                }
            }
        }

        if failures == 0 && additions == 0 {
            if success {
                eprintln!("All {total} tests succeeded");
                return Ok(());
            } else {
                break;
            }
        } else {
            if failures != 0 {
                let existing = total - additions;
                eprintln!("{failures} out of {existing} tests failed");
            }
            if additions != 0 {
                eprintln!("{additions} new tests found");
            }
            eprintln!("run `namaka review` to review the pending snapshots");
            exit(if failures != 0 { 1 } else { 2 });
        }
    }

    stderr().write_all(&output.stderr)?;
    Err(eyre!("unknown error"))
}
