use std::path::PathBuf;
use std::sync::mpsc;
use std::{fs, thread};

pub fn handle_in_thread(tx: mpsc::Sender<usize>, files: Vec<PathBuf>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let mut result = 0;

        for file in files {
            if let Ok(s) = fs::read_to_string(file) {
                result += s.split('\n').count();
            };
        }

        tx.send(result).unwrap();
    })
}
