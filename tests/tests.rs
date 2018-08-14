extern crate cargo_ws_release;

use cargo_ws_release::data_models::level::Level;
use cargo_ws_release::do_batch_release;
use std::env::*;
use std::fs::File;
use std::path::Path;

#[test]
fn do_batch_release_returns_dependency_graph() {
    let project_dir = current_dir().unwrap().to_str().unwrap().to_owned();
    let test_resource_dir = format!("{}/tests/resources", project_dir);
    assert!(set_current_dir(Path::new(&test_resource_dir)).is_ok());
    let test_cargo_file_path = format!("{}/Cargo.toml", test_resource_dir);
    let cargo_file = File::open(test_cargo_file_path).expect("Failed to open test Cargo.toml");

    let dep_graph = do_batch_release(cargo_file, &Level::Major);

    assert!(dep_graph.nodes.contains_key("lockchain-server"));
    assert!(
        dep_graph
            .nodes
            .get("lockchain-http")
            .unwrap()
            .deps
            .first()
            .unwrap()
            .name == "lockchain-core"
    )
}

#[test]
fn parse_level_from_str() {
    let levels = vec![
        ("major", Some(Level::Major)),
        ("minor", Some(Level::Minor)),
        ("patch", Some(Level::Patch)),
        ("rc", Some(Level::RC)),
        ("beta", Some(Level::Beta)),
        ("alpha", Some(Level::Alpha)),
        ("invalid_level", None),
    ];

    levels
        .iter()
        .for_each(|(level_str, level)| assert_eq!(*level, Level::from_str(level_str)));
}
