extern crate cargo_ws_release;

use cargo_ws_release::data_models::level::Level;
use cargo_ws_release::do_batch_release;
use cargo_ws_release::utilities::cargo_utils::bump_version;
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
        .for_each(|(level_str, level)| assert_eq!(Level::from_str(level_str), *level));
}

#[test]
fn bump_version_with_level() {
    let versions = vec![
        ("0.9.9", Level::Major, "1.0.0"),
        ("0.9.9", Level::Minor, "0.10.0"),
        ("0.9.9", Level::Patch, "0.9.10"),
        ("1.9.9", Level::Alpha, "1.9.10-alpha.1"),
        ("1.9.9", Level::Beta, "1.9.10-beta.1"),
        ("1.9.9", Level::RC, "1.9.10-rc.1"),
        ("1.9.9-dev", Level::Alpha, "1.9.9-alpha.1"),
        ("1.9.9-test", Level::Beta, "1.9.9-beta.1"),
        ("1.9.9-stg", Level::RC, "1.9.9-rc.1"),
        ("1.9.9-dev.1", Level::Alpha, "1.9.9-alpha.1"),
        ("1.9.9-test.4", Level::Beta, "1.9.9-beta.1"),
        ("1.9.9-stg.2", Level::RC, "1.9.9-rc.1"),
        ("2.3.9-alpha.8", Level::Alpha, "2.3.9-alpha.9"),
        ("2.3.9-beta.7", Level::Beta, "2.3.9-beta.8"),
        ("2.3.9-rc.6", Level::RC, "2.3.9-rc.7"),
        ("2.3.9-alpha.8.5.6.7.4", Level::Alpha, "2.3.9-alpha.9"),
    ];

    versions
        .iter()
        .for_each(|(given_version, given_level, expected_version)| {
            assert_eq!(
                bump_version(given_version, given_level).unwrap(),
                expected_version.to_string()
            )
        });
}

#[test]
fn bump_version_invalid_version_error() {
    let invalid_versions = vec!["a.b.c", "1.x.0", "1.", ".0.1", "1.1", "", " 1. 2. 3"];

    invalid_versions
        .iter()
        .for_each(|v| assert!(bump_version(v, &Level::Major).is_err()));
}
