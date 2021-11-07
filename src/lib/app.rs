use std::path;
use std::sync::mpsc;
use std::io::Result;
use std::thread::JoinHandle;

use super::counter::Counter;
use super::threads::handle_in_thread;


/// An Application used to count lines programmatically.
pub struct App {
    njobs: usize,
}


impl App {
    /// Creates a new `App`.
    ///
    /// # Arguments
    ///
    /// - `njobs` - The number of jobs ([std::thread::Thread])
    /// the application should run on.
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
    /// - Ok([usize]) - The total line count.
    /// - Err([std::io::Error]) - The error, if any.
    ///
    /// # Note
    /// Currently skips over any files containing non `UTF-8` encoded
    /// strings, as well as the directories `target/*` and `.git/*`.
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
    /// // Counts all files in the current dir.
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
    /// // Counts all lines in `/project/src/main.rs`.
    /// if let Ok(count) = app.count("/project/src/main.rs") {
    ///     println!("{} lines", count);
    /// } else {
    ///     println!("Something went wrong.");
    /// }
    /// ```
    pub fn count(&self, path: &str) -> Result<usize> {
        let target = path::PathBuf::from(path);
        let mut counter = Counter::new(target);
        let mut position = 0;
        let mut total = 0;

        let nfiles = counter.count_files()?;
        let workloads = counter.generate_workloads(self.njobs, nfiles)?;
        let files = counter.files;
        let (tx, rx) = mpsc::channel();

        let handles = workloads.iter().map(|load| {
            let start = position;
            let end = position + load;
            position = end;

            handle_in_thread(
                tx.clone(), files[start..end].to_vec()
            )
        }).collect::<Vec<JoinHandle<()>>>();

        for _ in handles {
            total += rx.recv().unwrap();
        }

        Ok(total)
    }
}
