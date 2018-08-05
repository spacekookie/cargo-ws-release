#[macro_use]
extern crate clap;
extern crate toml;
extern crate toml_edit;

use clap::{App, Arg};
use std::fs::File;
use std::{env, process};

mod cargo_utils;
mod graph;
mod utils;

use graph::DepGraph;

fn main() {
    let matches = App::new("cargo-ws-release")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("Level")
                .short("l")
                .long("level")
                .takes_value(true)
                .possible_values(&["major", "minor", "patch", "rc", "beta", "alpha"])
                .required(true)
                .help("Defines the release level"),
        )
        .get_matches();

    let level = matches.value_of("Level").unwrap();
    match File::open("Cargo.toml") {
        Ok(f) => do_batch_release(f, &level),
        _ => {
            eprintln!("No `Cargo.toml` found to work with!");
            process::exit(2);
        }
    }
}

fn do_batch_release(f: File, lvl: &str) {
    let members = cargo_utils::get_members(f);
    let configs = cargo_utils::batch_load_configs(&members);

    let v = configs
        .iter()
        .map(|c| cargo_utils::parse_config(c, &members))
        .fold(DepGraph::new(), |mut graph, (name, deps)| {
            graph.add_node(name.clone());

            deps.iter()
                .fold(graph, |graph, dep| graph.add_dependency(&name, dep))
        });

    println!("{:#?}", v);
}
