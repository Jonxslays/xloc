mod cli;
mod counter;


fn main() {
    let mut total = 0;
    let counter = counter::Counter::new();
    let parser = cli::Parser::new();

    if let Some(jobs) = parser.matches.value_of("jobs") {
        let _ = jobs;

        if let Ok(amount) = counter.count_lines(0, counter.count_files().unwrap()) {
            total += amount;
        }

        println!("Total lines: {}", total);
    }
}
