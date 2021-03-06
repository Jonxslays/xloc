mod parser;

use std::process;

use parser::Parser;
use xloc::App;

fn main() {
    let parser = Parser::new();
    let app = App::new(parser.njobs, parser.words);

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

#[cfg(test)]
mod main_tests {
    use super::main;

    #[test]
    fn main_test() {
        assert_eq!(main(), ());
    }
}
