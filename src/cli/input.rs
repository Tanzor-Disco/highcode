use clap::{Parser, ValueEnum};
use std::path::PathBuf;

#[derive(ValueEnum, Clone, Debug)]
pub enum ThemeChoice {
    Default,
    Light,
    VSCode,
}

#[derive(Parser, Debug)]
pub struct Args {
    pub file_path: PathBuf,

    #[arg(long,value_enum,default_value_t=ThemeChoice::Default)]
    pub theme: ThemeChoice,
}

pub fn read_args() -> Args {
    let args = Args::parse();
    args
}
