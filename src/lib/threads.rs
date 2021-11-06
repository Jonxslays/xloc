use std::path::PathBuf;
use std::{fs, thread};
use std::sync::mpsc;


pub fn handle_in_thread(tx: mpsc::Sender<usize>, files: Vec<PathBuf>) -> thread::JoinHandle<()> {
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
