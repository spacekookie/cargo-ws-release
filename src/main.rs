extern crate cargo_ws_release;
#[macro_use]
extern crate clap;
extern crate toml;
extern crate toml_edit;

use cargo_ws_release::data_models::level::*;
use cargo_ws_release::do_batch_release;
use clap::{App, Arg};
use std::fs::File;
use std::process;

fn main() {
    let matches = App::new("cargo-ws-release")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("Level")
                .short("l")
                .long("level")
                .takes_value(true)
                .possible_values(&[MAJOR, MINOR, PATCH, RC, BETA, ALPHA])
                .required(true)
                .help("Defines the release level"),
        )
        .arg(
            Arg::with_name("Crate")
                .help("Specifies the name of the crate to be released. If none specified all the crates will be bumped."),
        )
        .get_matches();

    let level = matches.value_of("Level").unwrap();
    let level = Level::from_str(level).unwrap();
    match File::open("Cargo.toml") {
        Ok(f) => {
            let _dep_graph = do_batch_release(f, &level);
        }
        _ => {
            eprintln!("No `Cargo.toml` found to work with!");
            process::exit(2);
        }
    }
}
