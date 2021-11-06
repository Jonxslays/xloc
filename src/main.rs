use std::path::PathBuf;
use std::{fs, thread};
use std::sync::mpsc;

mod cli;
mod counter;

fn handle_in_thread(tx: mpsc::Sender<usize>, files: Vec<PathBuf>) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        let mut result = 0;

        for file in files {
            let content = fs::read_to_string(file).unwrap();
            let split: Vec<&str> = content.split("\n").collect();
            result += split.len();
        }

        tx.send(result).unwrap();
    });

    handle
}

fn main() {
    let mut counter = counter::Counter::new(None);
    let parser = cli::Parser::new();
    let mut total: usize = 0;

    let (tx, rx) = mpsc::channel();

    if let Some(jobs) = parser.matches.value_of("jobs") {
        let njobs = jobs.parse::<usize>().unwrap();
        let nfiles = counter.count_files().unwrap();
        let workloads = counter.generate_workloads(njobs, nfiles).unwrap();
        let files = counter.files;
        let mut position: usize = 0;
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];

        for load in workloads {
            handles.push(
                handle_in_thread(
                    tx.clone(), files[position..position + load].to_owned()
                )
            );
            position += load;
        }

        for _ in handles {
            total += rx.recv().unwrap();
        }

        println!("The total lines are: {}", total);
    }
}
