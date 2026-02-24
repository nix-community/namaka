use std::{
    ffi::OsStr,
    fs::{read_dir, remove_file, rename, File},
    io::{self, stderr, BufRead, Write},
    path::Path,
    process::exit,
    thread::sleep,
    time::Duration,
};

use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use eyre::{eyre, Result};
use owo_colors::OwoColorize;
use similar::{Algorithm, ChangeTag, TextDiff};

use crate::{
    cfg::Config,
    cli::Opts,
    cmd::run::nix_eval,
    proto::{Snapshot, TestOutput},
};

pub fn review(opts: Opts, cfg: Option<Config>) -> Result<()> {
    let output = nix_eval(opts, cfg)?;
    let _ = ctrlc::set_handler(|| {
        let mut term = Term::stderr();
        let _ = term.show_cursor();
        let _ = writeln!(term, "interrupted");
        exit(0);
    });

    for line in output.stderr.lines() {
        let line = line?;
        let Some(line) = line.strip_prefix("trace: namaka=") else {
            continue;
        };

        let output = serde_json::from_str::<TestOutput>(line)?;
        let snapshots = output.dir.join("_snapshots");

        for entry in read_dir(snapshots.join(".pending"))? {
            use bstr::ByteSlice;

            let entry = entry?;
            let name = entry.file_name();

            if <[u8] as ByteSlice>::from_os_str(&name).map_or(false, |name| name.starts_with(b"."))
            {
                continue;
            };

            let old = snapshots.join(&name);
            let new = entry.path();
            let new_snap = Snapshot::parse(File::open(&new)?)?;
            println!();

            if let Ok(old_snap) = File::open(&old) {
                match (Snapshot::parse(old_snap), new_snap) {
                    (Ok(Snapshot::Json(old_value)), Snapshot::Json(new_value)) => {
                        print_diff(
                            "json",
                            &serde_json::to_string_pretty(&old_value)?,
                            &serde_json::to_string_pretty(&new_value)?,
                        )?;
                    }
                    (Ok(Snapshot::Pretty(old_value)), Snapshot::Pretty(new_value)) => {
                        print_diff("pretty", &old_value, &new_value)?;
                    }
                    (Ok(Snapshot::String(old_value)), Snapshot::String(new_value)) => {
                        print_diff("string", &old_value, &new_value)?;
                    }
                    (Ok(old_snap), new_snap) => {
                        old_snap.print_old()?;
                        new_snap.print_new()?;
                    }
                    (Err(e), new_snap) => {
                        println!("  {} failed to parse: {e}", "old".bold().red());
                        new_snap.print_new()?;
                    }
                }
                ask(&name, &old, &new)?;
            } else {
                println!("  {}: N/A", "old".bold().red());
                new_snap.print_new()?;
                ask(&name, &old, &new)?;
            }
        }

        return Ok(());
    }

    stderr().write_all(&output.stderr)?;
    Err(eyre!("unknown error"))
}

fn print_diff(fmt: &'static str, old: &str, new: &str) -> Result<()> {
    let diff = TextDiff::configure().algorithm(Algorithm::Patience).diff_lines(old, new);
    for change in diff.iter_all_changes() {
        let tag = change.tag();
        let change = change.to_string_lossy();
        match tag {
            ChangeTag::Equal => print!("{change}"),
            ChangeTag::Delete => print!("{}", change.bold().red()),
            ChangeTag::Insert => print!("{}", change.bold().green()),
        }
    }

    if !diff.newline_terminated() {
        println!("⏎");
    }

    println!(
        "{}\n---",
        format_args!("({fmt}) {} | {}", "old".red(), "new".green()).bold(),
    );

    Ok(())
}

fn ask(name: &OsStr, old: &Path, new: &Path) -> Result<()> {
    let choice = Select::with_theme(&ColorfulTheme::default())
        .item("accept".green())
        .item("reject".red())
        .item("skip".blue())
        .default(0)
        .with_prompt(format!("Review {}", name.to_string_lossy()))
        .interact()
        .inspect_err(|dialoguer::Error::IO(e)| {
            if e.kind() == io::ErrorKind::Interrupted {
                // make sure ctrlc handler has reset the terminal
                sleep(Duration::from_millis(16));
                exit(0);
            }
        })?;

    match choice {
        0 => rename(new, old).map_err(Into::into),
        1 => remove_file(new).map_err(Into::into),
        2 => Ok(()),
        _ => Err(eyre!("invalid selection")),
    }
}
