use eyre::Result;
use serde::Deserialize;

use std::{env::set_current_dir, fs::File, io::Read, path::PathBuf};

#[derive(Deserialize)]
pub struct Config {
    pub dir: Option<PathBuf>,
    pub check: Option<Check>,
    pub eval: Option<Eval>,
}

#[derive(Deserialize)]
pub struct Check {
    pub cmd: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct Eval {
    pub cmd: Option<Vec<String>>,
}

pub fn load() -> Result<Option<Config>> {
    let Ok(mut file) = File::open("namaka.toml") else {
        return Ok(None);
    };

    let mut buf = String::with_capacity(
        file.metadata()
            .map(|metadata| metadata.len() as usize)
            .unwrap_or_default(),
    );
    file.read_to_string(&mut buf)?;

    let cfg: Config = toml::from_str(&buf)?;

    if let Some(ref dir) = cfg.dir {
        set_current_dir(dir)?;
    }

    Ok(Some(cfg))
}
