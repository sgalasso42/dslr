use clap::{Arg, App};

#[derive(Debug, PartialEq)]
pub struct Config {
    pub dataset: String,
}

impl Config {
    pub fn new() -> Self {
        let matches = App::new("dslr")
            .version("0.1.0")
            .author("Simon Galasso <simon.galasso@hotmail.fr>")
            .about("Solve a polynomial equation")
            .arg(Arg::with_name("file")
                .required(true)
                .help("datasets file"))
            .get_matches();
        return Self {
            dataset: matches.value_of("file").unwrap_or("").to_string(),
        }
    }
}