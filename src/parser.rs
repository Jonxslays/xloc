use clap::{App, Arg, ArgMatches};


#[allow(dead_code)]
pub struct Parser<'a> {
    pub matches: ArgMatches<'a>
}


#[allow(dead_code)]
impl<'a> Parser<'a> {
    pub fn new() -> Self {
        let cli = App::new("xloc")
            .version("0.1.0")
            .about("A fast, multi-threaded line counting utility.")
            .arg(
                Arg::with_name("jobs")
                .short("j")
                .long("jobs")
                .value_name("NUM")
                .help("The number of jobs (threads) to run")
                .takes_value(true)
                .default_value("1")
            );

        Self { matches: cli.get_matches() }
    }
}
