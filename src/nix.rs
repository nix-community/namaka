use eyre::{eyre, Result};

use std::{
    ffi::OsString,
    fs::canonicalize,
    path::PathBuf,
    process::{Command, Output},
};

pub fn run_checks(dir: &Option<PathBuf>, cmd: Option<Vec<OsString>>) -> Result<Output> {
    if let Some(cmd) = cmd {
        let mut cmd = cmd.iter();
        Command::new(cmd.next().ok_or_else(|| eyre!("no command"))?)
            .args(cmd)
            .output()
            .map_err(Into::into)
    } else {
        let mut cmd = Command::new("nix");
        cmd.arg("flake")
            .arg("check")
            .arg("--extra-experimental-features")
            .arg("flakes nix-command");

        if let Some(dir) = dir {
            cmd.arg(canonicalize(dir)?);
        }

        cmd.output().map_err(Into::into)
    }
}

pub fn eval_checks(dir: &Option<PathBuf>, cmd: Option<Vec<OsString>>) -> Result<Output> {
    if let Some(cmd) = cmd {
        let mut cmd = cmd.iter();
        Command::new(cmd.next().ok_or_else(|| eyre!("no command"))?)
            .args(cmd)
            .output()
            .map_err(Into::into)
    } else {
        let mut cmd = Command::new("nix");
        cmd.arg("eval")
            .arg("--extra-experimental-features")
            .arg("flakes nix-command");

        if let Some(dir) = dir {
            let mut dir = canonicalize(dir)?.into_os_string();
            dir.push("#checks");
            cmd.arg(dir);
        } else {
            cmd.arg(".#checks");
        }

        cmd.output().map_err(Into::into)
    }
}
