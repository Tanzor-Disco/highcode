mod cli;
mod error;
mod highlighter;
mod render;
mod source;
mod theme;
mod tree;

use cli::input::read_args;
use cli::output::macos::copy_to_clipboard;
use highlighter::highlighter::Highlighter;
use render::html::render_html;
use source::read::read_file;
use tree::finder::finder::Finder;
use tree::parser::parser::SourceParser;

fn main() {
    // reading cli arguments
    let args = read_args();
    let file = match read_file(&args.file_path) {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };
    let theme = theme::theme::get_theme(&args.theme);

    // parsing a file
    let mut source_parser = match SourceParser::new(&file) {
        Ok(val) => val,
        Err(err) => {
            println!("{}", err);
            return;
        }
    };

    let tree = match source_parser.get_tree() {
        Ok(val) => val,
        Err(err) => {
            println!("couldn't get the tree: {}", err);
            return;
        }
    };

    // finding nodes in a file
    let finder = match Finder::new(&file, &tree) {
        Ok(val) => val,
        Err(err) => {
            println!("couldn't create finder: {}", err);
            return;
        }
    };
    let found_nodes = finder.find();

    // setting up highlight for the found nodes
    let highlighter = Highlighter::new(&found_nodes);
    let highlight_elements = match highlighter.highlight() {
        Ok(val) => val,
        Err(err) => {
            println!("couldn't highlight: {}", err);
            return;
        }
    };

    // rendering html
    let render = render_html(&highlight_elements, &file.text, &theme);
    copy_to_clipboard(&render, &file.text);
    println!("Added code to clipboard");
}
