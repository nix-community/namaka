use std::{
    fs::File,
    io::{BufRead, BufReader, Read, Write},
};

use bat::PrettyPrinter;
use eyre::{eyre, Result};
use owo_colors::OwoColorize;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "kebab-case", tag = "format", content = "value")]
pub enum Snapshot {
    Json(Value),
    Pretty(String),
    String(String),
}

impl Snapshot {
    pub fn parse(file: File) -> Result<Self> {
        let file = &mut BufReader::new(file);
        let header = file
            .lines()
            .next()
            .ok_or_else(|| eyre!("failed to parse"))??;

        match header.as_str() {
            "#json" => Ok(Self::Json(serde_json::from_reader(file)?)),
            "#pretty" => {
                let mut value = String::new();
                file.read_to_string(&mut value)?;
                Ok(Self::Pretty(value))
            }
            "#string" => {
                let mut value = String::new();
                file.read_to_string(&mut value)?;
                Ok(Self::String(value))
            }
            _ => Err(eyre!("invalid header {header}")),
        }
    }

    pub fn print_old(self) -> Result<()> {
        println!("---");
        let fmt = self.format();
        self.print()?;
        println!("{} ({fmt})", "↑ old".bold().red());
        Ok(())
    }

    pub fn print_new(self) -> Result<()> {
        println!("{} ({})", "↓ new".bold().green(), self.format());
        self.print()?;
        println!("---");
        Ok(())
    }

    pub fn to_writer(&self, mut w: impl Write) -> Result<()> {
        writeln!(w, "#{}", self.format())?;
        match self {
            Self::Json(value) => serde_json::to_writer(w, &value).map_err(Into::into),
            Self::Pretty(value) => write!(w, "{value}").map_err(Into::into),
            Self::String(value) => write!(w, "{value}").map_err(Into::into),
        }
    }

    fn format(&self) -> &'static str {
        match self {
            Self::Json(_) => "json",
            Self::Pretty(_) => "pretty",
            Self::String(_) => "string",
        }
    }

    fn print(self) -> Result<()> {
        let newline = match self {
            Self::Json(value) => {
                let value = serde_json::to_vec_pretty(&value)?;
                PrettyPrinter::new()
                    .language("json")
                    .input_from_bytes(&value)
                    .print()?;
                value.ends_with(b"\n")
            }
            Self::Pretty(value) => {
                PrettyPrinter::new()
                    .language("nix")
                    .input_from_bytes(value.as_bytes())
                    .print()?;
                value.ends_with('\n')
            }
            Self::String(value) => {
                PrettyPrinter::new()
                    .input_from_bytes(value.as_bytes())
                    .print()?;
                value.ends_with('\n')
            }
        };

        if !newline {
            println!("⏎");
        }

        Ok(())
    }
}
