//! # xloc
//!
//! - A fast, multi-threaded line counting utility. xloc hopes to speed up
//! in places where other tools slow down.
//! - An easy to use API is available through `xloc::App` if you would
//! like to count lines from within another Rust project.
//! - Simple and intuitive command line interface.
//!
//! ---
//!
//! #### Getting started from the command line
//!
//! ```sh
//! ## Verify xloc is working.
//! $ xloc --version
//!
//! ## Get help.
//! $ xloc --help
//!
//! ## Recursively count lines in the current dir, with 12 jobs.
//! $ xloc -j 12 .
//!
//! ## Count lines in `test.txt`, with 1 job.
//! $ xloc ./test.txt
//!
//! ## Count lines in `main.py` and the `scripts` dir, with 6 jobs.
//! $ xloc -j 6 main.py scripts
//! ```
//!
//! ## Getting started in your own project
//!
//! Check out our documentation on `xloc::App` below.

mod app;
mod counter;
mod threads;

pub use app::App;
