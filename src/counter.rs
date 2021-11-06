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

    fn count_chunk(&self, size: usize, start: usize) -> Result<usize> {
        let mut result = 0;

        for idx in start..(start + size) {
            let content = fs::read_to_string(&self.files.lock().unwrap()[idx])?;
            let split: Vec<&str> = content.split("\n").collect();
            result += split.len();
        }

        Ok(result)
    }

    fn generate_workloads(&self, njobs: usize, nfiles: usize) -> Result<Vec<usize>> {
        let chunk_size = nfiles / njobs;
        let remainder = nfiles % njobs;

        let mut workloads = vec![chunk_size; njobs];

        for i in 0..remainder {
            workloads[i] += 1;
        }

        Ok(workloads)
    }

    pub fn count_files(&self) -> Result<usize> {
        self.scan(&self.curdir)?;
        Ok(self.files.lock().unwrap().len())
    }

    pub fn count(&self, njobs: usize) -> Result<usize> {
        let mut total = 0;
        let nfiles = self.count_files()?;
        let workloads = self.generate_workloads(njobs, nfiles)?;
        let mut position = 0;

        for load in workloads {
            if let Ok(amount) = self.count_chunk(load, position) {
                total += amount;
                position += load;
            }
        }

        Ok(total)
    }
}
