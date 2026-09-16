mod cli;
pub mod error;
mod source;

use cli::input::read_args;
use source::read;

fn main() {
    let args = read_args();
    let file = match read::read_file(&args.file_path) {
        Ok(val) => val,
        Err(err) => {
            println!("{:?}", err);
            return;
        }
    };
    println!("{:?}", file);
}
