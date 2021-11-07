use xloc::App;
mod parser;


fn main() {
    let parser = parser::Parser::new();

    if let Some(jobs) = parser.matches.value_of("jobs") {
        let njobs = jobs.parse::<usize>().unwrap_or(1);
        let app = App::new(njobs);

        match app.count(None) {
            Ok(count) => println!("{}", count),
            Err(e) => println!("{}", e),
        }
    }
}
