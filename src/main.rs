use clap::App;
use clap::{crate_name, crate_version, crate_description};

fn main() {
    App::new(crate_name!())
        .version(crate_version!())
        .about(crate_description!())
        .get_matches();

    println!("Hello, world!");
}
