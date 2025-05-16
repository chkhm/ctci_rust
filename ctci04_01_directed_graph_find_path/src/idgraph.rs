use std::collections::HashSet;

struct Node {
    value: i32,
    neighbors: Vec<usize>,
}

struct Graph {
    nodes: Vec<Node>,
}

fn new_node(g: &mut Graph, value: i32) -> usize {
    let n = Node {
        value,
        neighbors: vec![],
    };
    g.nodes.push(n);

    g.nodes.len() - 1
}

fn new_graph() -> Graph {
    Graph { nodes: vec![] }
}

fn add_edge(g: &mut Graph, node1: usize, node2: usize) {
    let n1 = &mut g.nodes[node1];
    n1.neighbors.push(node2);
}

fn get_neighbors(g: &Graph, n: usize) -> Vec<i32> {
    let mut result = Vec::new();
    let node = &g.nodes[n];
    let cn = &node.neighbors;
    for n in cn {
        result.push(g.nodes[*n].value);
    }
    result
}

fn get_value(g: &Graph, n: usize) -> i32 {
    g.nodes[n].value
}

fn print_nodes(g: &Graph) {
    for (n, _) in g.nodes.iter().enumerate() {
        let v = get_value(g, n);
        let cn = get_neighbors(g, n);
        println!("v: {}, neighbors : {:?}", v, cn);
    }
}

fn find_path(g: &Graph, start_node: usize, end_node: usize) -> bool {
    let mut visited_nodes = HashSet::new();
    let mut path = Vec::new();

    let found_path = find_directed_path(g, start_node, end_node, &mut visited_nodes, &mut path);

    println!("Result: found a path: {} path is {:?}", found_path, path);
    found_path
}

fn find_directed_path(
    g: &Graph,
    start_node: usize,
    end_node: usize,
    visited_nodes: &mut HashSet<usize>,
    path: &mut Vec<usize>,
) -> bool {
    if start_node == end_node {
        return true;
    }
    if visited_nodes.contains(&start_node) {
        return false;
    }
    visited_nodes.insert(start_node);
    path.push(start_node);
    let n1 = &g.nodes[start_node];
    for n in n1.neighbors.iter() {
        let result = find_directed_path(g, *n, end_node, visited_nodes, path);
        if result == true {
            return true;
        } else {
            path.pop();
        }
    }
    false
}
