use std::path;
use std::thread;
use std::sync::mpsc;
use std::io::Result;

use super::counter;
use super::threads;


pub struct App {}


impl App {
    pub fn new() -> Self {
        Self {  }
    }

    pub fn count(&self, njobs: usize, path: Option<&str>) -> Result<usize> {
        let mut target: path::PathBuf;
        let mut handles: Vec<thread::JoinHandle<()>> = vec![];
        let mut position: usize = 0;
        let mut total: usize = 0;

        if let Some(p) = path {
            target = path::PathBuf::new();
            target.push(p);
        } else {
            target = std::env::current_dir()?;
        }

        let mut counter = counter::Counter::new(target);
        let nfiles = counter.count_files()?;
        let workloads = counter.generate_workloads(njobs, nfiles)?;
        let files = counter.files;
        let (tx, rx) = mpsc::channel();

        for load in workloads {
            handles.push(
                threads::handle_in_thread(
                    tx.clone(), files[position..position + load].to_owned()
                )
            );
            position += load;
        }

        for _ in handles {
            total += rx.recv().unwrap();
        }

        Ok(total)
    }
}
