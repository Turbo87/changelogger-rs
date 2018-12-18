use std::env;
use std::path::Path;
use std::process;

use clap::{App, Arg};
use clap::{crate_name, crate_version, crate_description};
use failure::Error;

fn main() -> Result<(), Error> {
    let path = env::current_dir()?.into_os_string();

    let matches = App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .arg(Arg::with_name("PATH")
            .help("path of a git repository")
            .index(1)
            .default_value_os(path.as_os_str()))
        .get_matches();

    let path = Path::new(matches.value_of("PATH").unwrap());
    if !path.exists() {
        println!("Path {:?} does not exist!", path);
        process::exit(1);
    }
    if !path.is_dir() {
        println!("Path {:?} is not a directory!", path);
        process::exit(1);
    }

    println!("PATH: {:?}", path);

    Ok(())
}
