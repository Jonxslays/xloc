use std::io::Result;
use std::path;
use std::sync::mpsc;

use super::counter::Counter;
use super::threads::{handle, handle_in_thread};

/// An Application used to count lines programmatically.
#[derive(Debug, Clone, Copy, Hash)]
pub struct App {
    njobs: usize,
    words: bool,
}

impl Default for App {
    /// ```
    /// // Creates a default App, that uses 1 thread and count lines.
    /// let app = xloc::App::default();
    ///
    /// assert_eq!(app.get_njobs(), 1);
    /// assert_eq!(app.get_words(), false);
    /// ```
    fn default() -> Self {
        Self {
            njobs: 1,
            words: false,
        }
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
    /// - `words` - If true, count words instead of lines.
    ///
    /// # Returns
    /// - [App] - The newly created `App`.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creates a new App, that will use 2 threads and count words.
    /// let app = xloc::App::new(2, true);
    /// ```
    ///
    /// ```
    /// // Creates a new App, that will use 12 thread and count lines.
    /// let app = xloc::App::new(12, false);
    /// ```
    pub fn new(njobs: usize, words: bool) -> Self {
        Self { njobs, words }
    }

    /// Counts the lines/words in a file, or recursively counts the
    /// lines/words in all files if a directory is passed to `path`.
    ///
    /// # Arguments
    /// - `path` - The path to run this function against.
    ///
    /// # Returns
    ///
    /// - [Result<usize, std::io::Error>] - The total line/word count or
    /// the error, if any.
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
    /// let app = xloc::App::default();
    ///
    /// // Counts lines in all files in the current dir and subdirs.
    /// match app.count(".") {
    ///     Ok(count) => println!("{} lines", count),
    ///     Err(e) => println!("Error: {}", e),
    /// }
    /// ```
    ///
    /// ```no_run
    /// // Runs in 12 threads, and counts words.
    /// let app = xloc::App::new(12, true);
    ///
    /// // Counts the words in `/project/src/main.rs`.
    /// if let Ok(count) = app.count("/project/src/main.rs") {
    ///     println!("{} words", count);
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
        if self.njobs == 1 {
            return Ok(self.adjust(handle(counter.files, self.words)));
        } else {
            njobs = self.njobs - 1;
        }

        // Generate an even distribution of workloads
        let mut total = 0;
        let mut position = 0;
        let workloads = counter.generate_workloads(njobs, nfiles)?;
        let files = counter.files;

        // Create a channel so threads can send data
        let (tx, rx) = mpsc::channel();

        // Create a thread for each workload
        for load in workloads {
            let start = position;
            let end = position + load;
            position = end;

            handle_in_thread(tx.clone(), files[start..end].to_vec(), self.words);
        }

        // Drop the final sender, so the receiver doesn't block the main thread
        drop(tx);

        // Receive the data from the threads
        for rcvd in rx {
            total += rcvd;
        }

        Ok(self.adjust(total))
    }

    fn adjust(&self, total: usize) -> usize {
        // If we are counting lines, we need to add 1 to the result.
        if !self.words {
            return total + 1;
        }

        total
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
    /// let mut app = xloc::App::default();
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
    /// // Creates a new `App` that uses 3 jobs.
    /// let app = xloc::App::new(3, false);
    ///
    /// assert_eq!(app.get_njobs(), 3);
    /// ```
    pub fn get_njobs(&self) -> usize {
        self.njobs
    }

    /// Gets whether or not we are counting words, instead of lines.
    ///
    /// # Returns
    /// - [bool] - Whether or not we are counting words.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creates a new `App`.
    /// let app = xloc::App::default();
    ///
    /// // By default, we do not count words.
    /// assert_eq!(app.get_words(), false);
    /// ```
    ///
    /// ```
    /// // Creates a new `App` with 3 jobs, and words set to true.
    /// let app = xloc::App::new(3, true);
    ///
    /// assert_eq!(app.get_words(), true);
    /// ```
    pub fn get_words(&self) -> bool {
        self.words
    }

    /// Sets whether or not to count words, instead of lines.
    ///
    /// # Arguments
    ///
    /// - `value` - Whether or not to count words.
    ///
    /// # Returns
    /// - [bool] - The updated state.
    ///
    /// # Examples
    ///
    /// ```
    /// // Creates a new mutable `App`.
    /// let mut app = xloc::App::default();
    ///
    /// assert_eq!(app.get_words(), false);
    ///
    /// // Set words to true, so we will now count words.
    /// app.set_words(true);
    ///
    /// assert_eq!(app.get_words(), true);
    /// ```
    pub fn set_words(&mut self, value: bool) -> bool {
        self.words = value;
        value
    }
}
