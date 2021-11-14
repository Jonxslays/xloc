//! # xloc
//!
//! - A fast, multi-threaded line counting utility. xloc hopes to speed up
//! in places where other tools slow down.
//! - An easy to use API is available through `xloc::App` if you would
//! like to count lines/words from within another Rust project.
//! - Simple and intuitive command line interface.
//!
//! ---
//!
//! #### Getting started from the command line
//!
//! ```bash
//! ## Verify xloc is working.
//! xloc --version
//!
//! ## Get help.
//! xloc --help
//!
//! ## Count lines for all files in the current dir, with 1 job.
//! xloc .
//!
//! ## Count words for all files in the current dir with nproc jobs.
//! xloc -wj $(nproc) .
//!
//! ## Count words for 1 file, with 1 job.
//! xloc -w test.txt
//!
//! ## Count lines for all files in the src dir, with 6 jobs.
//! xloc -j 6 src
//! ```
//!
//! ## Getting started in your own project
//!
//! Check out our documentation on `xloc::App` below.

mod app;
mod counter;
mod threads;

pub use app::App;
