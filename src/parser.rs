use clap::{App, Arg};

pub struct Parser {
    pub njobs: usize,
    pub paths: Vec<String>,
    pub words: bool,
}

impl Parser {
    pub fn new() -> Self {
        let cli = App::new("xloc")
            .version("0.1.0")
            .about("A fast, multi-threaded line counting utility")
            .arg(
                Arg::with_name("jobs")
                    .short("j")
                    .long("jobs")
                    .value_name("NUM")
                    .help("The number of jobs (threads) to run")
                    .takes_value(true)
                    .default_value("1"),
            )
            .arg(
                Arg::with_name("words")
                    .short("w")
                    .long("--words")
                    .help("If included, count words instead of lines")
                    .takes_value(false),
            )
            .arg(
                Arg::with_name("path")
                    .help("The path or paths to parse")
                    .takes_value(true)
                    .multiple(true)
                    .required(true)
                    .default_value("."),
            );

        let matches = cli.get_matches();
        let words = matches.is_present("words");

        let paths = matches
            .values_of("path")
            .unwrap()
            .map(|p| p.to_string())
            .collect::<Vec<String>>();

        let njobs = matches
            .value_of("jobs")
            .unwrap()
            .parse::<usize>()
            .unwrap_or(1);

        Self {
            njobs,
            paths,
            words,
        }
    }
}

#[cfg(test)]
mod parser_tests {
    use super::Parser;

    #[test]
    fn new_parser() {
        let parser = Parser::new();
        assert_eq!(parser.njobs, 1);
        assert_eq!(parser.paths, vec!["."]);
        assert_eq!(parser.words, false);
    }

    #[test]
    fn mut_parser() {
        let mut parser = Parser::new();
        parser.njobs = 3;
        parser.paths = vec!["tests/data".to_owned()];
        parser.words = true;

        assert_eq!(parser.njobs, 3);
        assert_eq!(parser.paths, vec!["tests/data"]);
        assert_eq!(parser.words, true);
    }
}
