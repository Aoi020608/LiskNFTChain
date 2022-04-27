//    9
// 6      12
//1 4    34  45
//

// inorder [1, 6, 4, 9, 34, 12, 45]
// preorder [9, 6, 1, 4, 12, 34, 45]
// postorder [1, 4, 6, 34, 45, 12, 9]

use std::collections::HashSet;
use std::collections::VecDeque;
#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Vertex(u32);

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
pub struct Edge(u32, u32);

#[derive(Clone)]
pub struct Graph {
    vertices: Vec<Vertex>,
    edges: Vec<Edge>,
}

impl Graph {
    pub fn new(vertices: Vec<Vertex>, edges: Vec<Edge>) -> Self {
        Graph { vertices, edges }
    }
}

impl From<u32> for Vertex {
    fn from(item: u32) -> Self {
        Vertex(item)
    }
}

impl Vertex {
    pub fn value(&self) -> u32 {
        self.0
    }

    pub fn neighbors(&self, graph: &Graph) -> VecDeque<Vertex> {
        graph
            .edges
            .iter()
            .filter(|e| e.0 == self.0)
            .map(|f| f.1.into())
            .collect()
    }
}

impl From<(u32, u32)> for Edge {
    fn from(item: (u32, u32)) -> Self {
        Edge(item.0, item.1)
    }
}

// preorder
pub fn depth_first_search_preorder(graph: &Graph, root: Vertex, objectives: Vertex) -> Option<Vec<u32>> {
    let mut visited = HashSet::new();
    let mut history = Vec::new();
    let mut queue = VecDeque::new();
    queue.push_back(root);

    while let Some(current_vertex) = queue.pop_front() {
        history.push(current_vertex.value());

        if current_vertex == objectives {
            return Some(history);
        }

        // For each over the neighbors of current vertex
        for neighbor in current_vertex.neighbors(graph).into_iter().rev() {
            // Insert in the HashSet of visiteds if this value not exist yet
            if visited.insert(neighbor) {
                // Add the neighbor on front of queue
                queue.push_front(neighbor);
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use crate::depth_first_search::*;

    #[test]
    fn find_1_fail() {
        /* Example graph #1:
         *
         *            (1)   <--- Root
         *           /   \
         *         (2)   (3)
         *        / |     | \
         *     (4) (5)   (6) (7)
         *
         *
         */

        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objectives = 99;

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search_preorder(&graph, root.into(), objectives.into()),
            None
        );
    }

    #[test]
    fn find_2_success() {
        let vertices = vec![1, 2, 3, 4, 5, 6, 7];
        let edges = vec![(1, 2), (1, 3), (2, 4), (2, 5), (3, 6), (3, 7)];

        let root = 1;
        let objectives = 7;

        let correct_path = vec![1, 2, 4, 5, 3, 6, 7];

        let graph = Graph::new(
            vertices.into_iter().map(|v| v.into()).collect(),
            edges.into_iter().map(|e| e.into()).collect(),
        );

        assert_eq!(
            depth_first_search_preorder(&graph, root.into(), objectives.into()),
            Some(correct_path)
        )
    }
}
