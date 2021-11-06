use std::fs;
use std::env;
use std::path::PathBuf;
use std::process;
use std::io::Result;

pub struct Counter {
    pub curdir: PathBuf
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

        Self { curdir: curdir }
    }

    fn scan(&self, dir: &PathBuf, mut buf: Vec<PathBuf>) -> Result<Vec<PathBuf>> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let path = entry?.path();

                match path.is_dir() {
                    true => {
                        if let Ok(paths) = self.scan(&path, buf.clone()) {
                            buf = paths;
                        }
                    },
                    false => buf.push(path),
                }
            }
        }

        Ok(buf)
    }

    // fn count_file(&self) -> i32 {
    //     return 0;
    // }

    pub fn count(&mut self) -> Result<i32> {
        let paths = self.scan(&self.curdir, Vec::new())?;
        // println!("{:?}", paths);
        println!("{:?}", paths.len());

        Ok(0)
    }
}
