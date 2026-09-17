mod cli;
mod error;
mod source;
mod parser;
mod highlighter;
mod theme;

use cli::input::read_args;
use source::read;
use parser::parser::SourceParser;

fn main() {
    let args = read_args();
    let file = match read::read_file(&args.file_path) {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let mut source_parser = match SourceParser::new(&file) {
        Ok(val) => val,
        Err(err) => {
            println!("{}",err);
            return;
        }
    };
    
    println!("{:?}",source_parser.get_tree().unwrap().root_node().to_sexp());
}
