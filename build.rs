use std::{
    env,
    fs::{create_dir_all, File},
    path::Path,
};

use clap::{CommandFactory, ValueEnum};
use clap_complete::{generate_to, Shell};
use clap_mangen::Man;

mod cli {
    include!("src/cli.rs");
}

fn main() {
    println!("cargo:rerun-if-env-changed=GEN_ARTIFACTS");

    if let Some(dir) = env::var_os("GEN_ARTIFACTS") {
        let out = &Path::new(&dir);
        create_dir_all(out).unwrap();
        let cmd = &mut cli::Opts::command();

        Man::new(cmd.clone())
            .render(&mut File::create(out.join("namaka.1")).unwrap())
            .unwrap();

        for subcmd in cmd.get_subcommands() {
            let name = format!("namaka-{}", subcmd.get_name());
            Man::new(subcmd.clone().name(&name))
                .render(&mut File::create(out.join(format!("{name}.1"))).unwrap())
                .unwrap();
        }

        for shell in Shell::value_variants() {
            generate_to(*shell, cmd, "namaka", out).unwrap();
        }
    }
}
