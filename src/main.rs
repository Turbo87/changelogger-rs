use std::env;

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

    let path = matches.value_of("PATH").unwrap();
    println!("PATH: {}", path);

    Ok(())
}
