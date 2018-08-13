extern crate cargo_ws_release;

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

    let dep_graph = do_batch_release(cargo_file, "major");

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
