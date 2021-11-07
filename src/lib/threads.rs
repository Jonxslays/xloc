use std::path::PathBuf;
use std::{fs, thread};
use std::sync::mpsc;


pub fn handle_in_thread(tx: mpsc::Sender<usize>, files: Vec<PathBuf>) -> thread::JoinHandle<()> {
    let handle = thread::spawn(move || {
        let mut result = 0;

        for file in files {
            match fs::read_to_string(file) {
                Ok(s) => result += s.split("\n").count(),
                _ => (),
            };
        }

        tx.send(result).unwrap();
    });

    handle
}
