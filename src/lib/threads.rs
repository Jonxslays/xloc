use std::path::PathBuf;
use std::sync::mpsc;
use std::{fs, thread};

use regex::Regex;

pub fn handle_in_thread(
    tx: mpsc::Sender<usize>,
    files: Vec<PathBuf>,
    words: bool,
) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        tx.send(handle(files, words)).unwrap();
    })
}

pub fn handle(files: Vec<PathBuf>, words: bool) -> usize {
    let mut result = 0;
    let pattern;

    if words {
        pattern = r"[a-zA-Z0-9]+";
    } else {
        pattern = r"\n";
    }

    let rgx = Regex::new(pattern).unwrap();

    for file in files {
        if let Ok(s) = fs::read_to_string(file) {
            result += rgx.find_iter(&s).count();
        }
    }

    result
}
