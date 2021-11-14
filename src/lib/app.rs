use std::io::Result;
use std::path;
use std::sync::mpsc;
use std::thread::JoinHandle;

use super::counter::Counter;
use super::threads::{handle, handle_in_thread};

/// An Application used to count lines programmatically.
#[derive(Debug, Clone, Copy, Hash)]
pub struct App {
    njobs: usize,
}

impl Default for App {
    fn default() -> Self {
        Self { njobs: 1 }
    }
}

impl App {
    /// Creates a new xloc `App`.
    ///
    /// # Arguments
    ///
    /// - `njobs` - The number of jobs ([std::thread::Thread])
    /// the application should run on.
    ///
    /// # Returns
    /// - [App] - The newly created `App`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creates a new App, that will use 2 threads.
    /// let app = xloc::App::new(2);
    /// ```
    pub fn new(njobs: usize) -> Self {
        Self { njobs }
    }

    /// Counts the lines in a file, or recursively counts the lines in
    /// all files if a directory is passed to `path`.
    ///
    /// # Arguments
    /// - `path` - The path to run this function against.
    ///
    /// # Returns
    ///
    /// - [Result<usize, std::io::Error>] - The total line count or the
    /// error, if any.
    ///
    /// # Note
    /// Currently skips over any files containing non `UTF-8` encoded
    /// characters, as well as the directories `target` and `.git`.
    ///
    /// Please open an [issue](https://github.com/Jonxslays/xloc/issues)
    /// if you have suggestions for more directories to ignore by
    /// default.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// // Runs in 1 thread.
    /// let app = xloc::App::new(1);
    ///
    /// // Counts lines in all files in the current dir and subdirs.
    /// match app.count(".") {
    ///     Ok(count) => println!("{} lines", count),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    ///
    /// ```no_run
    /// // Runs in 12 threads.
    /// let app = xloc::App::new(12);
    ///
    /// // Counts the lines in `/project/src/main.rs`.
    /// if let Ok(count) = app.count("/project/src/main.rs") {
    ///     println!("{} lines", count);
    /// } else {
    ///     println!("Something went wrong.");
    /// }
    /// ```
    pub fn count(&self, path: &str) -> Result<usize> {
        let target = path::PathBuf::from(path);
        let mut counter = Counter::new(target);
        let nfiles = counter.count_files()?;
        let njobs: usize;

        // If only 1 job, no need to even create threads
        // Otherwise decrement njobs by 1 to save 1 job
        // for the main thread
        match self.njobs {
            1 => return Ok(handle(counter.files)),
            _ => {
                njobs = self.njobs - 1;
            }
        }

        // Otherwise calculate thread loads and handle in threads
        let mut total = 0;
        let mut position = 0;
        let workloads = counter.generate_workloads(njobs, nfiles)?;
        let files = counter.files;
        let (tx, rx) = mpsc::channel();

        let handles = workloads
            .iter()
            .map(|load| {
                let start = position;
                let end = position + load;
                position = end;

                handle_in_thread(tx.clone(), files[start..end].to_vec())
            })
            .collect::<Vec<JoinHandle<()>>>();

        for _ in handles {
            total += rx.recv().unwrap();
        }

        Ok(total)
    }

    /// Sets the number of jobs ([std::thread::Thread]) the `App` should
    /// use.
    ///
    /// # Arguments
    ///
    /// - `njobs` - The new number of jobs.
    ///
    /// # Returns
    /// - [usize] - The new number of jobs.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creates a new mutable `App` that uses 1 job.
    /// let mut app = xloc::App::new(1);
    ///
    /// assert_eq!(app.get_njobs(), 1);
    ///
    /// // Sets the number of jobs to 6.
    /// app.set_njobs(6);
    ///
    /// assert_eq!(app.get_njobs(), 6);
    /// ```
    pub fn set_njobs(&mut self, njobs: usize) -> usize {
        self.njobs = njobs;
        njobs
    }

    /// Gets the number of jobs ([std::thread::Thread]) the `App` is
    /// currently set to use.
    ///
    /// # Returns
    /// - [Result<usize>] - The current number of jobs.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creates a new mutable `App` that uses 1 job.
    /// let app = xloc::App::new(1);
    ///
    /// assert_eq!(app.get_njobs(), 1);
    /// ```
    pub fn get_njobs(&self) -> usize {
        self.njobs
    }
}
