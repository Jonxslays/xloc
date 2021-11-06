use std::fs;
use std::path::PathBuf;
use std::io::Result;


pub struct Counter {
    pub curdir: PathBuf,
    pub files: Vec<PathBuf>
}


impl Counter {
    pub fn new(curdir: PathBuf) -> Self {
        Self { curdir, files: vec![] }
    }

    fn scan(&mut self, dir: &PathBuf) -> Result<()> {
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
                    false => self.files.push(path),
                }
            }
        }

        Ok(())
    }

    pub fn generate_workloads(&self, njobs: usize, nfiles: usize) -> Result<Vec<usize>> {
        let chunk_size = nfiles / njobs;
        let remainder = nfiles % njobs;

        let mut workloads = vec![chunk_size; njobs];

        for i in 0..remainder {
            workloads[i] += 1;
        }

        Ok(workloads)
    }

    pub fn count_files(&mut self) -> Result<usize> {
        self.scan(&self.curdir.clone())?;
        Ok(self.files.len())
    }
}
