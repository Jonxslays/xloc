mod parser;

use std::process;

use parser::Parser;
use xloc::App;

fn main() {
    let parser = Parser::new();
    let app = App::new(parser.njobs);

    let total = parser
        .paths
        .iter()
        .map(|p| match app.count(p) {
            Ok(count) => count,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        })
        .sum::<usize>();

    println!("{}", total);
}
