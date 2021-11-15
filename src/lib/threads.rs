use std::path::PathBuf;
use std::sync::mpsc;
use std::{fs, thread};

use regex::Regex;

pub fn handle_in_thread(tx: mpsc::Sender<usize>, files: Vec<PathBuf>, words: bool) {
    thread::spawn(move || {
        tx.send(handle(files, words)).unwrap();
    });
}

pub fn handle(files: Vec<PathBuf>, words: bool) -> usize {
    let mut result = 0;
    let pattern;

    if words {
        pattern = r#"(\w?\\?['"]?\w+\\?['"]?(\w+)?)+"#;
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

#[cfg(test)]
mod threads_tests {
    use std::sync::mpsc;
    use std::{path::PathBuf, str::FromStr};

    use super::handle;
    use super::handle_in_thread;

    #[test]
    fn threads_handle_lines() {
        let path = vec![PathBuf::from_str("tests/data/data.rs").unwrap()];
        let result = handle(path, false);
        assert_eq!(result, 16);
    }

    #[test]
    fn threads_handle_words() {
        let path = vec![PathBuf::from_str("tests/data/data.rs").unwrap()];
        let result = handle(path, true);
        assert_eq!(result, 36);
    }

    #[test]
    fn threads_handle_in_thread_lines() {
        let path = vec![PathBuf::from_str("tests/data/data.rs").unwrap()];
        let (tx, rx) = mpsc::channel();
        handle_in_thread(tx.clone(), path, false);
        let result = rx.recv().unwrap();
        assert_eq!(result, 16);
    }

    #[test]
    fn threads_handle_in_thread_words() {
        let path = vec![PathBuf::from_str("tests/data/data.rs").unwrap()];
        let (tx, rx) = mpsc::channel();
        handle_in_thread(tx.clone(), path, true);
        let result = rx.recv().unwrap();
        assert_eq!(result, 36);
    }
}
