use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum ThemeChoice {
    Dark,
    Light,
}

#[derive(Parser, Debug)]
#[command(name = "HighCode", about = "Syntax highlighting for source code")]
pub struct Args {
    /// Path to the source code file
    pub file_path: PathBuf,

    /// Color theme used for syntax highlighting
    #[arg(long,value_enum,default_value_t=ThemeChoice::Dark)]
    pub theme: ThemeChoice,
}

pub fn read_args() -> Args {
    let args = Args::parse();
    args
}
