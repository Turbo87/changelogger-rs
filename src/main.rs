use std::env;
use std::path::Path;
use std::process;

use clap::{App, Arg};
use clap::{crate_name, crate_version, crate_description};
use failure::{Error, format_err};
use git2::Repository;

fn main() {
    if let Err(err) = run_app() {
        eprintln!("error: {}", err);
        process::exit(1);
    }
}

fn run_app() -> Result<(), Error> {
    let path = env::current_dir()?.into_os_string();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("PATH")
            .help("path of a git repository")
            .index(1)
            .default_value_os(path.as_os_str()))
        .arg(Arg::with_name("FROM")
            .long("from")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("TO")
            .long("to")
            .takes_value(true)
            .required(true))
        .get_matches();

    let path = Path::new(matches.value_of("PATH").unwrap());
    if !path.exists() {
        return Err(format_err!("path '{}' does not exist", path.to_string_lossy()));
    }
    if !path.is_dir() {
        return Err(format_err!("path '{}' is not a directory", path.to_string_lossy()));
    }

    let repo = Repository::open(path)
        .map_err(|err| format_err!("{}", err.message()))?;

    println!("PATH: {:?}", repo.path());

    Ok(())
}
