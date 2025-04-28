// -----------------------------------------------------------------------------------------------
// Module rcgraph
//
// This module uses Rc and RefCell to manage the relationships between nodes and edges
//

use std::sync::atomic::{AtomicUsize, Ordering};
use std::{cell::RefCell, collections::HashSet, rc::Rc};

fn get_id() -> usize {
    static COUNTER: AtomicUsize = AtomicUsize::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

pub struct Node {
    id: usize,
    value: i32,
    edges: Vec<Edge>,
}

pub struct Edge {
    // id: usize,
    weight: i32,
    destination: Rc<RefCell<Node>>,
}

pub struct Graph {
    // id: usize,
    nodes: Vec<Rc<RefCell<Node>>>,
}

pub fn new_graph() -> Graph {
    Graph {
        // id: get_id(),
        nodes: vec![],
    }
}

pub fn new_node(g: &mut Graph, value: i32) -> Rc<RefCell<Node>> {
    let n = Rc::new(RefCell::new(Node {
        id: get_id(),
        value,
        edges: vec![],
    }));
    g.nodes.push(n.clone());
    n
}

pub fn add_edge(_g: &mut Graph, node1: Rc<RefCell<Node>>, node2: Rc<RefCell<Node>>, weight: i32) {
    node1.borrow_mut().edges.push(Edge {
        // id: get_id(),
        weight,
        destination: node2,
    });
}

pub fn print_nodes(g: &Graph) {
    for n in g.nodes.iter() {
        print!("val: {}, edges: ", n.borrow().value);
        for e in n.borrow().edges.iter() {
            print!("(w: {} d: {}) ", e.weight, e.destination.borrow().value);
        }
        println!();
    }
}

pub fn find_path(g: &Graph, start_node: Rc<RefCell<Node>>, dest_node: Rc<RefCell<Node>>) -> bool {
    let mut visited_nodes = HashSet::new();
    let mut path = Vec::new();

    find_recursive_path(g, start_node, dest_node, &mut visited_nodes, &mut path)
}

pub fn find_recursive_path(
    _g: &Graph,
    start_node: Rc<RefCell<Node>>,
    dest_node: Rc<RefCell<Node>>,
    visited_nodes: &mut HashSet<usize>,
    path: &mut Vec<usize>,
) -> bool {
    if start_node.borrow().id == dest_node.borrow().id {
        return true;
    }
    // contains does not work because we would need to have the trait hash implemented
    //if visited_nodes.contains(start_node) {}
    let start_node_id = start_node.borrow().id;
    if visited_nodes.iter().any(|i| *i == start_node_id) {
        return false;
    }
    visited_nodes.insert(start_node_id);
    path.push(start_node_id);

    for edge in start_node.borrow().edges.iter() {
        let result = find_recursive_path(
            _g,
            edge.destination.clone(),
            dest_node.clone(),
            visited_nodes,
            path,
        );
        if result {
            return true;
        } else {
            path.pop();
        }
    }
    false
}
