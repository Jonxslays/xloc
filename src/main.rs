mod cli;
mod counter;
use std::thread;
use std::sync::{Arc, Mutex};

fn generate_thread_loads(njobs: usize, counter:counter::Counter) -> Vec<usize> {
    let count = counter.count_files().unwrap();
    let initial = count / njobs;
    let remainder = count % njobs;

    let mut distribution = vec![initial; njobs];

    for i in 0..remainder {
        distribution[i] += 1;
    }

    distribution
}


fn main() {
    let mut total = Arc::new(Mutex::new(0));
    let counter = counter::Counter::new();
    let parser = cli::Parser::new();

    if let Some(jobs) = parser.matches.value_of("jobs") {
        let mut threads: Vec<thread::Thread> = vec![];
        let njobs = jobs.parse::<usize>().unwrap_or(1);
        let work = generate_thread_loads(njobs, counter);
        let mut idx = 0; // starting point for the first thread

        for load in work {
            let t = thread::spawn(|| {
                // borrowing issues if trying to use counter here.
            });
        }
    }
}
