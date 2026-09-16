use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
pub struct Args {
    pub file_path: PathBuf,
}

pub fn read_args() -> Args {
    let args = Args::parse();
    args
}
