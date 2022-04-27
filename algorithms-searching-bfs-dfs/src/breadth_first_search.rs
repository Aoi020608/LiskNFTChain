use std::collections::HashSet;
use std::collections::VecDeque;

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Node(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    #[allow(dead_code)]
    nodes: Vec<Node>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(nodes: Vec<Node>, edges: Vec<Edge>) -> Self {
        Graph { nodes, edges }
    }
}

impl From<u32> for Node {
    fn from(item: u32) -> Self {
        Node(item)
    }
}

impl Node {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> Vec<Node> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|e| e.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}

pub fn breadth_first_search(graph: &Graph, root: Node, target: Node) -> Option<Vec<u32>> {
    let mut visited: HashSet<Node> = HashSet::new();
    let mut history: Vec<u32> = Vec::new();
    let mut queue = VecDeque::new();

    visited.insert(root);
    queue.push_back(root);
    while let Some(current_node) = queue.pop_front() {
        history.push(current_node.value());

        // If we reach the goal, return our travel history
        if current_node == target {
            return Some(history);
        }

        // Check the neighboring nodes for any that we've not visited yet.
        for neighbor in current_node.neighbors(graph) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                queue.push_back(neighbor);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use crate::breadth_first_search::*;

    /* Example graph #1:
     *
     *            (1)   <--- Root
     *           /   \
     *         (2)   (3)
     *        / |     | \
     *     (4) (5)   (6) (7)
     *          |
     *         (8)
     */
    fn graph1() -> Graph {
        let nodes = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7), (5, 8)];

        Graph::new(
            nodes.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        )
    }

    #[test]
    fn breadth_first_search_graph1_when_node_not_found_returns_node() {
        let graph = graph1();
        let root = 1;
        let target = 10;

        assert_eq!(
            breadth_first_search(&graph, root.into(), target.into()),
            None
        );
    }

    #[test]
    fn breadth_first_search_graph1_when_target_8_should_evaluate_all_nodes_first() {
        let graph = graph1();
        let root = 1;
        let target = 8;

        let expected_path = vec![1, 2, 3, 4, 5, 6, 7, 8];

        assert_eq!(
            breadth_first_search(&graph, root.into(), target.into()),
            Some(expected_path)
        );
    }
}

// #[derive(Debug)]
// struct Node {
//     value: i32,
//     left: Option<Box<Node>>,
//     right: Option<Box<Node>>,
// }

// impl Node {
//     pub fn new(value: i32) -> Self {
//         Node {
//             left: None,
//             right: None,
//             value,
//         }
//     }

//     pub fn insert(&mut self, value: i32) {
//         if self.value > value {
//             // left node
//             match self.left {
//                 None => {
//                     self.left = Some(Box::new(Node {
//                         value,
//                         left: None,
//                         right: None,
//                     }))
//                 }
//                 Some(ref mut node) => node.insert(value),
//             }
//         } else {
//             match self.right {
//                 None => {
//                     self.right = Some(Box::new(Node {
//                         value,
//                         left: None,
//                         right: None,
//                     }))
//                 }
//                 Some(ref mut node) => node.insert(value),
//             }
//         }
//     }

//     pub fn lookup(&self, value: i32) -> bool {
//         if self.value == value {
//             return true;
//         }
//         if self.value > value {
//             match self.left {
//                 None => false,
//                 Some(ref node) => node.lookup(value),
//             }
//         } else {
//             match self.right {
//                 None => false,
//                 Some(ref node) => node.lookup(value),
//             }
//         }
//     }

//     pub fn breadth_search() {
//         let current_node =
//     }
// }
