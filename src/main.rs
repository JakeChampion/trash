use glob::glob;
use std::{env, path::PathBuf};

fn main() {
    // Forcibly disable backtraces.
    env::remove_var("RUST_LIB_BACKTRACE");
    env::remove_var("RUST_BACKTRACE");

    let mut app = clap::App::new("trash");
    app = app.version("1.0.0");
    app = app.about("Move files and folders to the trash");
    app = app.arg(
        clap::Arg::with_name("files")
            .takes_value(true)
            .multiple(true)
            .help("Files to process")
            .required(true),
    );
    // Ignore all flags of `rm` program.
    for flag in &["r", "f", "i", "d", "P", "R", "v", "W"] {
        app = app.arg(
            clap::Arg::with_name(flag)
                .takes_value(false)
                .multiple(false)
                .short(flag)
                .hidden(true)
                .required(false),
        );
    }
    let globs: Vec<String> = app
        .get_matches()
        .values_of("files")
        .map_or_else(Vec::new, |v| {
            v.map(::std::convert::From::from)
                .map(|glob: String| {
                    if glob == "."
                        || glob == ".."
                        || glob == "./"
                        || glob == "../"
                        || glob.ends_with("/.")
                        || glob.ends_with("/..")
                        || glob.ends_with("/./")
                        || glob.ends_with("/../")
                    {
                        eprintln!(r#"Trash: "." and ".." may not be moved to the trash"#);
                        std::process::exit(exitcode::USAGE);
                    } else if glob == "/" {
                        eprintln!(r#"Trash: "/" may not be moved to the trash"#);
                        std::process::exit(exitcode::USAGE);
                    } else {
                        glob
                    }
                })
                .collect::<Vec<String>>()
        });

    for pattern in globs {
        let paths: Vec<PathBuf> = glob(&pattern).unwrap().filter_map(Result::ok).collect();
        trash::delete_all(paths).unwrap();
    }
}
