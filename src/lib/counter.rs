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
            let mut excluded = false;

            for exclude in ["target", ".git"] {
                excluded = !d.as_ref().unwrap().path().ends_with(exclude);

                if excluded {
                    break;
                }
            }

            excluded
        }) {
            let path = entry?.path();

            if path.is_dir() {
                self.scan(&path)?;
            } else {
                self.files.push(path);
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

#[cfg(test)]
mod counter_tests {
    use std::{path::PathBuf, str::FromStr};

    use super::Counter;

    #[test]
    fn counter_new() {
        let path = PathBuf::from_str("tests/data").unwrap();
        let counter = Counter::new(path.clone());

        assert_eq!(counter.path, path);
        assert_eq!(counter.files.len(), 0);
    }

    #[test]
    fn counter_scan_dir() {
        let path = PathBuf::from_str("tests/data").unwrap();
        let mut counter = Counter::new(path.clone());

        let result = counter.scan(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn counter_scan_file() {
        let path = PathBuf::from_str("tests/data/data.rs").unwrap();
        let mut counter = Counter::new(path.clone());

        let result = counter.scan(&path);
        assert!(result.is_ok());
    }

    #[test]
    fn counter_count_files() {
        let path = PathBuf::from_str("tests/data").unwrap();
        let mut counter = Counter::new(path.clone());

        let result = counter.count_files();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 3);
    }

    #[test]
    fn counter_count_file() {
        let path = PathBuf::from_str("tests/data/data.rs").unwrap();
        let mut counter = Counter::new(path.clone());

        let result = counter.count_files();
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 1);
    }

    #[test]
    fn counter_generate_workloads() {
        let path = PathBuf::from_str("tests/data").unwrap();
        let counter = Counter::new(path.clone());

        let result = counter.generate_workloads(1, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![3]);

        let result = counter.generate_workloads(3, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 1, 1]);

        let result = counter.generate_workloads(2, 3);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![2, 1]);
    }
}
