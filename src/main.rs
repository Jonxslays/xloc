use std::process;

mod cli;
mod counter;

// fn generate_thread_loads(njobs: usize, counter: &counter::Counter) -> Vec<usize> {
//     let count = counter.count_files().unwrap();
//     let initial = count / njobs;
//     let remainder = count % njobs;

//     let mut distribution = vec![initial; njobs];

//     for i in 0..remainder {
//         distribution[i] += 1;
//     }

//     distribution
// }

fn main() {
    let counter = counter::Counter::new();
    let parser = cli::Parser::new();

    if let Some(jobs) = parser.matches.value_of("jobs") {
        let njobs = jobs.parse::<usize>().unwrap();
        let total = match counter.count(njobs) {
            Ok(t) => t,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            }
        };

        println!("Total: {}", total);
    }
}
