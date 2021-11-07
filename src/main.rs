mod parser;

use std::process;

use xloc::App;
use parser::Parser;


fn main() {
    let parser = Parser::new();
    let app = App::new(parser.jobs);

    let total = parser.path.iter().map(|p| {
        match app.count(p) {
            Ok(count) => count,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            },
        }
    }).sum::<usize>();

    println!("{}", total);
}
