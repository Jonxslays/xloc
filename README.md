# xloc

A fast, multi-threaded line counting utility written in Rust.

## What is xloc

- A drop in replacement for bash's `wc -l`.

- Your project has x lines of code, xloc gets the value of x for you.

- xloc is itended to be used from the command line. You can use it to
count the number of lines in a file, or aggregate the total number of
lines of all files in a directory.

- While command line utility was the focus, a public API has also been made available to use in your own rust projects in the form of `xloc::App`.

- By default xloc will ignore any
directory named `target` or `.git`. This will likely be configurable
at a later date.

## Getting started

xloc supports Rust version 1.24.0 and greater.

For more information, read the [API Reference](https://docs.rs/xloc).

---

### Installation

#### From the command line

```bash
cargo install xloc
```

#### As a package dependency

```toml
# Cargo.toml
[dependencies]
xloc = "0.1.0"
```

---

### Usage

#### On the command line
```bash
# Count lines for all files in the current dir, with 1 job.
xloc .

# Count lines for 1 file, with 12 jobs.
xloc -j 12 test.txt

# Count lines for all files in the src dir, with 6 jobs.
xloc -j 6 src
```

#### In a file
```rs
// main.rs
use xloc::App;

fn main() {
    // Create a mutable `App` using 1 job.
    let mut app = App::new(1);
    assert_eq!(app.get_njobs(), 1)

    // Set the number of jobs to 12.
    app.set_njobs(12);
    assert_eq!(app.get_njobs(), 12);

    // Recursively count lines in the current dir.
    match app.count(".") {
        Ok(count) => println!("{} lines", count),
        Err(e) => println!("Error: {}", e),
}
```

---

## License

The xloc crate for Rust is licensed under the [BSD 3-Clause License](https://github.com/Jonxslays/xloc/blob/main/LICENSE).
