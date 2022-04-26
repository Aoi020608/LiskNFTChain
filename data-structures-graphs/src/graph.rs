use std::collections::{HashMap, HashSet};
use std::fmt;

#[derive(Debug, Clone)]
pub struct NodeNotInGraph;

impl fmt::Display for NodeNotInGraph {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "accessing a node that is not in the graph")
    }
}

pub trait Graph {
    fn new() -> Self;

    fn adjancey_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>>;

    fn adjancey_table(&self) -> &HashMap<String, Vec<(String, i32)>>;

    fn add_node(&mut self, node: &str) -> bool {
        match self.adjancey_table().get(node) {
            None => {
                self.adjancey_table_mutable()
                    .insert((*node).to_string(), Vec::new());
                true
            }
            _ => false,
        }
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjancey_table_mutable()
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });
    }

    fn neighbors(&self, node: &str) -> Result<&Vec<(String, i32)>, NodeNotInGraph> {
        match self.adjancey_table().get(node) {
            None => Err(NodeNotInGraph),
            Some(i) => Ok(i),
        }
    }

    fn contains(&self, node: &str) -> bool {
        self.adjancey_table().get(node).is_some()
    }

    fn nodes(&self) -> HashSet<&String> {
        self.adjancey_table().keys().collect()
    }

    fn edges(&self) -> Vec<(&String, &String, i32)> {
        let mut edges = Vec::new();
        for (from_node, from_node_neighbors) in self.adjancey_table() {
            for (to_node, weight) in from_node_neighbors {
                edges.push((from_node, to_node, *weight));
            }
        }
        edges
    }
}

pub struct DirectedGraph {
    adjancey_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for DirectedGraph {
    fn new() -> DirectedGraph {
        DirectedGraph {
            adjancey_table: HashMap::new(),
        }
    }

    fn adjancey_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjancey_table
    }

    fn adjancey_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjancey_table
    }
}

pub struct UndirectedGraph {
    adjancey_table: HashMap<String, Vec<(String, i32)>>,
}

impl Graph for UndirectedGraph {
    fn new() -> UndirectedGraph {
        UndirectedGraph {
            adjancey_table: HashMap::new(),
        }
    }

    fn adjancey_table_mutable(&mut self) -> &mut HashMap<String, Vec<(String, i32)>> {
        &mut self.adjancey_table
    }

    fn adjancey_table(&self) -> &HashMap<String, Vec<(String, i32)>> {
        &self.adjancey_table
    }

    fn add_edge(&mut self, edge: (&str, &str, i32)) {
        self.add_node(edge.0);
        self.add_node(edge.1);

        self.adjancey_table
            .entry(edge.0.to_string())
            .and_modify(|e| {
                e.push((edge.1.to_string(), edge.2));
            });

        self.adjancey_table
            .entry(edge.1.to_string())
            .and_modify(|e| {
                e.push((edge.0.to_string(), edge.2));
            });
    }
}
