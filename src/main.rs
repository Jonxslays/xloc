mod cli;

fn main() {
    let parser = cli::Parser::new();

    if let Some(jobs) = parser.matches.value_of("jobs") {
        if let Ok(njobs) = jobs.parse::<usize>() {
            println!("We would run {} jobs", njobs);
        } else {
            println!("We will run 1 job cause they passed bad input.");
        }
    }
}
