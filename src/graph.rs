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

impl Node {
    /// Returns an `Option<&Dep>` if the node depends on it
    fn depends_on(&self, name: &str) -> bool {
        self.deps.iter().fold(false, |mut acc, dep| {
            if dep.name == name {
                acc = true;
            }

            acc
        })
    }
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

    pub fn add_dependency(mut self, name: &str, dep: &Dep) -> Self {
        self.nodes.get_mut(name).unwrap().deps.push(dep.clone());
        self
    }

    /// Goes through the graph and finds all packages that depend on `name`
    pub fn find_depending(&self, name: &str) -> Vec<&Node> {
        self.nodes.iter().fold(Vec::new(), |mut vec, (_, pkg)| {
            if pkg.depends_on(name) {
                vec.push(pkg);
            }

            vec
        })
    }

    /// Find the ideal node to start publishing from
    ///
    /// This is a node which ideally has no dependencies in the graph
    /// while many nodes depend on it.
    pub fn find_optimal(&self) {
        /// A simple local structure to tag nodes with
        #[derive(Debug)]
        struct NodeExt<'a> {
            node: &'a Node,
            rds: usize,
        }

        // Iterate through all nodes
        let nodes = self
            .nodes
            .iter()
            // Fold into lists of `NodeExt` tags
            .fold(Vec::<NodeExt>::new(), |vec, (_, node)| {
                vec.insert(
                    // Iterate through all nodes again (This is O(N²) !)
                    self.nodes
                        .iter()
                        // Fold search into `NodeExt` - increments rds if dependency found
                        .fold(NodeExt { node, rds: 0 }, |ne, (_, onode)| NodeExt {
                            rds: ne.rds + onode.depends_on(&node.name) as usize,
                            ..ne
                        }),
                )
            });

        // for (_, node) in &self.nodes {
        //     let mut ne = NodeExt { node, rds: 0 };

        //     for onode in &self.nodes {
        //         if onode.1.depends_on(&node.name) {
        //             ne.rds += 1;
        //         }
        //     }

        //     nodes.push(ne);
        // }

        println!("{:#?}", nodes);
    }
}

/// A utility trait that makes inserting into collections more functional
/// 🥁
trait InsertSelf<T> {
    fn insert(self, i: T) -> Self;
}

impl<T> InsertSelf<T> for Vec<T> {
    fn insert(mut self, i: T) -> Self {
        self.push(i);
        self
    }
}
