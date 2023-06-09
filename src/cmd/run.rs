use std::{
    ffi::{OsStr, OsString},
    process::{Command, Output},
};

use eyre::{eyre, Result};

use crate::{cfg::Config, cli::Opts};

pub fn nix_check(opts: Opts, cfg: Option<Config>) -> Result<Output> {
    run(
        opts.cmd,
        (|| cfg?.check?.cmd)(),
        "nix",
        [
            "flake",
            "check",
            "--extra-experimental-features",
            "flakes nix-command",
        ],
    )
}

pub fn nix_eval(opts: Opts, cfg: Option<Config>) -> Result<Output> {
    run(
        opts.cmd,
        (|| cfg?.eval?.cmd)(),
        "nix",
        [
            "eval",
            ".#checks",
            "--extra-experimental-features",
            "flakes nix-command",
        ],
    )
}

fn run<const N: usize>(
    cli_cmd: Option<Vec<OsString>>,
    cfg_cmd: Option<Vec<String>>,
    default_cmd: &str,
    default_args: [&str; N],
) -> Result<Output> {
    let mut cmd = if let Some(cli_cmd) = cli_cmd {
        let mut args = cli_cmd.iter();
        cmd(args.next().ok_or_else(|| eyre!("no command"))?, args)
    } else if let Some(cfg_cmd) = cfg_cmd {
        let mut args = cfg_cmd.iter();
        cmd(args.next().ok_or_else(|| eyre!("no command"))?, args)
    } else {
        cmd(default_cmd, default_args)
    };
    cmd.output().map_err(Into::into)
}

fn cmd(cmd: impl AsRef<OsStr>, args: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Command {
    let mut cmd = Command::new(cmd);
    cmd.args(args);
    cmd
}
