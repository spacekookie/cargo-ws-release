//! Internal dependency graph
//!
//! We have a selection of members and dependencies between those. When
//! bumping a version number, we need to bump _all_ version numbers
//! referencing it.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Dep {
    pub name: String,
    pub version: String,
}

#[derive(Debug)]
pub struct Node {
    pub name: String,
    pub deps: Vec<Dep>,
}

#[derive(Debug)]
pub struct DepGraph {
    pub nodes: HashMap<String, Node>,
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

    pub fn add_dependency(mut self, name: &str, dep: &Dep) -> Self {
        self.nodes.get_mut(name).unwrap().deps.push(dep.clone());
        self
    }
}
