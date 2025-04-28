# Cracking the Coding Interview Chapter 4 Question 1 - find out if there is a path between two nodes in a directed graph

You are given a directed graph and two nodes in that graph. Create an algorithm that finds out if there is a path between two nodes.

**Clarification:** Does it matter if there is only a path from node `n2` to node `n1` but not the other direction?
For this specific sitation we assume we are ok with a path in only one direction, no matter wether fron `n1` to `n2` or vice versa.

## Approach 1: Simplified Djiekstra depth search or breadth search

Remarks: In Rust there is the pet graph library that provides data structures, travels and other algorithms for managing graphs. For the sake of the exercise
let's first ignore the library and do it all by hand.

We have to create a struct Graph and a struct Node and some traits to do the searching. For the sake of simplicity we first do this non-generic and then extend it to a generic data structure.
For the traits, we define our algorithm as a trait of graph that searches for a path between two nodes.

```

Edge = pair<Node, Node>
NodeMarker = Hashset<Node>

Graph {
    attr nodes : LinkedList<Node>,
    attr edges : LinkedList<Edge>,
}

Node {
    attr value : T
    attr neighbors : List<Node>
}

// depth search
fn find_directed_path(n1 : &Node, n2 : &Node, visited_nodes : &NodeMarker) -> bool {
    if (n1 == n2) {
        return true;
    }
    if (n1 in visited_nodes) {
        return false
    }
    visited_nodes.add(n1);
    for each n in n1.neighbors {
        rslt = find_directed_path(n, n2);
        if rslt == true return true;
    }
    return false;
}

```