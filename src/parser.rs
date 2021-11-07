use clap::{App, Arg};


pub struct Parser {
    pub jobs: usize,
    pub path: Vec<String>,
}


impl Parser {
    pub fn new() -> Self {
        let cli = App::new("xloc")
            .version("0.1.0")
            .about("A fast, multi-threaded line counting utility.")
            .arg(Arg::with_name("jobs")
                .short("j")
                .long("jobs")
                .value_name("NUM")
                .help("The number of jobs (threads) to run")
                .takes_value(true)
                .default_value("1")
            )
            .arg(Arg::with_name("path")
                .help("The path or paths to parse")
                .takes_value(true)
                .multiple(true)
                .default_value(".")
            );

        let matches = cli.get_matches();

        let path = matches
            .values_of("path")
            .unwrap()
            .map(|p| p.to_string())
            .collect::<Vec<String>>();

        let jobs = matches
            .value_of("jobs")
            .unwrap()
            .parse::<usize>()
            .unwrap_or(1);

        Self { jobs, path }
    }
}
