use std::fs;
use std::env;
use std::path::PathBuf;
use std::process;
use std::io::Result;
use std::sync::{Arc, Mutex};

pub struct Counter {
    pub curdir: PathBuf,
    pub files: Arc<Mutex<Vec<PathBuf>>>
}

impl Counter {
    pub fn new() -> Self {
        let curdir = match env::current_dir() {
            Ok(p) => p,
            Err(e) => {
                println!("{}", e);
                process::exit(1);
            },
        };

        Self { curdir: curdir, files: Arc::new(Mutex::new(vec![]))}
    }

    fn scan(&self, dir: &PathBuf) -> Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)?
                .filter(|d| {
                    for exclude in vec!["target", ".git"] {
                        let path = d.as_ref().unwrap().path();

                        if path.ends_with(exclude) {
                            return false
                        }
                    }
                    return true
                }
            ) {
                let path = entry?.path();

                match path.is_dir() {
                    true => self.scan(&path)?,
                    false => self.files.lock().unwrap().push(path),
                }
            }
        }

        Ok(())
    }

    pub fn count_files(&self) -> Result<usize> {
        self.scan(&self.curdir)?;
        Ok(self.files.lock().unwrap().len())
    }

    pub fn count_lines(&self, start: usize, end: usize) -> Result<usize> {
        let mut result = 0;

        for idx in start..end {
            let content = fs::read_to_string(&self.files.lock().unwrap()[idx])?;
            let split: Vec<&str> = content.split("\n").collect();
            result += split.len();
        }

        Ok(result)
    }
}
