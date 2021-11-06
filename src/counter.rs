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

            for entry in fs::read_dir(dir)?.filter(|d| {
                for exclude in vec!["target", ".git"] {
                    if d.as_ref().unwrap().path().ends_with(exclude) {
                        return false;
                    }
                }

                return true;
            }) {
                let path = entry?.path();

                match path.is_dir() {
                    true => self.scan(&path)?,
                    false => self.files.lock().unwrap().push(path),
                }
            }
        }

        Ok(())
    }

    pub fn count(&self) -> Result<i32> {
        self.scan(&self.curdir)?;
        println!("Total files: {}", self.files.lock().unwrap().len());

        Ok(0)
    }
}
