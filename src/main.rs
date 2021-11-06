mod cli;
mod counter;
use std::process;

fn main() {
    let parser = cli::Parser::new();

    if let Some(jobs) = parser.matches.value_of("jobs") {
        if let Ok(njobs) = jobs.parse::<usize>() {
            println!("We would run {} jobs", njobs);
        } else {
            println!("We will run 1 job cause they passed bad input.");
        }
    }

    let mut counter = counter::Counter::new();
    match counter.count() {
        Ok(_) => (),
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };
}
