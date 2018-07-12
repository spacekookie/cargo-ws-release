//! Internal dependency graph
//!
//! We have a selection of members and dependencies between those. When
//! bumping a version number, we need to bump _all_ version numbers
//! referencing it.

use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub deps: Vec<String>,
}

#[derive(Debug)]
pub struct DepGraph {
    nodes: HashMap<String, Node>,
}

impl DepGraph {
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
        }
    }

    pub fn add_node(&mut self, name: String) {
        self.nodes.insert(
            name.clone(),
            Node {
                name,
                deps: Vec::new(),
            },
        );
    }

    pub fn add_dependency(mut self, name: &str, dep: &str) -> Self {
        self.nodes.get_mut(name).unwrap().deps.push(dep.into());
        self
    }
}
