extern crate toml_edit;

use std::fs::File;
use std::{env, process};

mod cargo_utils;
mod graph;
mod utils;

use graph::DepGraph;

fn main() {
    // if env::args().len() < 2 {
    //     eprintln!("Usage: cargo ws-release <major|minor|patch|rc|beta|alpha>");
    //     process::exit(255);
    // }

    // let level = env::args().next().unwrap();
    match File::open("Cargo.toml") {
        Ok(f) => do_batch_release(f, "yolo"),
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

    // println!("{:#?}", v.find_depending("lockchain-core"));

    v.find_optimal();
}
