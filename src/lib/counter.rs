use std::fs;
use std::io::Result;
use std::path::{Path, PathBuf};

pub struct Counter {
    pub path: PathBuf,
    pub files: Vec<PathBuf>,
}

impl Counter {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            files: vec![],
        }
    }

    fn scan(&mut self, path: &Path) -> Result<()> {
        if path.is_file() {
            self.files.push(path.to_owned());
            return Ok(());
        }

        for entry in fs::read_dir(path)?.filter(|d| {
            for exclude in ["target", ".git"] {
                let path = d.as_ref().unwrap().path();

                if path.ends_with(exclude) {
                    return false;
                }
            }

            true
        }) {
            let path = entry?.path();

            match path.is_dir() {
                true => self.scan(&path)?,
                false => self.files.push(path),
            }
        }

        Ok(())
    }

    pub fn generate_workloads(&self, njobs: usize, nfiles: usize) -> Result<Vec<usize>> {
        let chunk_size = nfiles / njobs;
        let remainder = nfiles % njobs;
        let mut workloads = vec![chunk_size; njobs];

        for i in workloads.iter_mut().take(remainder) {
            *i += 1;
        }

        Ok(workloads)
    }

    pub fn count_files(&mut self) -> Result<usize> {
        self.scan(&self.path.clone())?;
        Ok(self.files.len())
    }
}
