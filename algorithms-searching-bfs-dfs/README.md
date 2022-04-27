# Algorithms - Binary Search

## Description

Binary Search Tree

Traversal
loops

1. Breadth First Search

2. Depth First Search

Hash table is not ordered.

1. Breadth Fist Search/Traversal
   shortest path

left -> right
use additional memory
need to track each node

Pros
Shortest Path
Closer Nodes

Cons
More Memory

         9
     6      12

1 4 34 45

[9, 6, 12, 1, 4, 34, 45]

2. Depth First Search
   Check to see if it exists

Pros
Less Memory
Does Path Exist?

Cons
Can Get Slow

         9
     6      12

1 4 34 45

[9, 6, 1, 4, 12, 34, 45]

Traversal
time complexity - O(N)

use case:
a solution is not far from the root of the tree: BFS

the tree is very deep and solutions are rare: BFS (DFS will take long)

tree is very wide: DFS(BFS will need too much memory)

solutions are frequent but located deep in the tree: DFS

determining whether a path exists between 2 nodes: DFS

finding the shortest path: BFS

Graph -

Dijkstra + Bellman-Ford Algorithms


## Resources

- https://stackoverflow.com/questions/9844193/what-is-the-time-and-space-complexity-of-a-breadth-first-and-depth-first-tree-tr
- https://medium.com/basecs/finding-the-shortest-path-with-a-little-help-from-dijkstra-613149fbdc8e
