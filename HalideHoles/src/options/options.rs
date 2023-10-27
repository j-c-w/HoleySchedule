use clap::{App,Arg};

pub struct Options {
    pub source: String
}

pub fn parse_options() -> Options {
    let args = App::new("Options")
        .version("1.0")
        .about("Synthesize holes in schedules")
        .arg(
            Arg::with_name("source")
            .help("Source file")
            .required(true)
            .index(1)
        ).get_matches();

    let opts: Options = Options {
        source: args.value_of("source").unwrap().into(),
    };
    opts
}
