extern crate toml;
extern crate toml_edit;

pub use data_models::graph;
use graph::DepGraph;
use std::fs::File;
pub use utilities::cargo_utils;
pub use utilities::utils;

pub mod data_models;
pub mod utilities;

pub fn do_batch_release(f: File, lvl: &str) -> DepGraph {
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
    v
}
