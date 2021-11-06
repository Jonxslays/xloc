use xloc::App;
mod parser;


fn main() {
    let parser = parser::Parser::new();
    let app = App::new();

    if let Some(jobs) = parser.matches.value_of("jobs") {
        let njobs = jobs.parse::<usize>().unwrap_or(1);

        match app.count(njobs, None) {
            Ok(count) => println!("{}", count),
            Err(e) => println!("{}", e),
        }
    }
}
