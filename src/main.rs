use glob::glob;
use std::path::PathBuf;
use trash;

fn main() {
    let app = clap::App::new("trash");
    let app = app.version("1.0.0");
    let app = app.about("Move files and folders to the trash");
    let app = app.arg(
        clap::Arg::with_name("files")
            .takes_value(true)
            .multiple(true)
            .help("Files to process")
            .required(true),
    );
    // Ignore all flags of `rm` program.
    let app = app.arg(
        clap::Arg::with_name("r")
            .takes_value(false)
            .multiple(false)
            .short("r")
            .aliases(&["f", "i", "d", "P", "R", "v", "W"])
            .hidden(true)
            .required(false),
    );
    let globs: Vec<String> = app
        .get_matches()
        .values_of("files")
        .map_or_else(Vec::new, |v| v.map(::std::convert::From::from).collect());

    for pattern in globs {
        let paths: Vec<PathBuf> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
        trash::delete_all(paths).unwrap();
    }
}
