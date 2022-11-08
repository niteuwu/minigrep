use std::env;
use minigrep;

fn main() {
    let args: Vec<String> = env::args().collect();
    let file = minigrep::File::new(args);
    minigrep::get_match(file.reads());
}

