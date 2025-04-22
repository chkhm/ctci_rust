use std::{cell::RefCell, collections::HashSet, rc::Rc};

struct Node {
    value : i32,
    edges : Vec<Edge>,
}

struct Edge {
    weight : i32,
    destination : Rc<RefCell<Node>>,
}

struct Graph {
    nodes : Vec<Rc<RefCell<Node>>>,
}

fn new_graph() -> Graph {
    Graph {
        nodes : vec![],
    }
}

fn new_node(g : &mut Graph, value : i32) -> Rc<RefCell<Node>> {
    let n = Rc::new(RefCell::new(Node {
        value,
        edges : vec![],
    }));
    g.nodes.push(n.clone());
    n
}

fn add_edge(g : &mut Graph, node1 : Rc<RefCell<Node>>, node2 : Rc<RefCell<Node>>, weight : i32) {

    node1.borrow_mut().edges.push(Edge {
        weight,
        destination : node2,
    });
}

fn print_nodes(g : &Graph) {
    for n in g.nodes.iter() {
        print!("val: {}, edges: ", n.borrow().value);
        for e in n.borrow().edges.iter() {
            print!("(w: {} d: {}) ", e.weight, e.destination.borrow().value);
        }
        println!();
    }
}

fn find_recursive_path(g : &Graph, start_node : Rc<RefCell<Node>>, dest_node : Rc<RefCell<Node>>, 
    visited_nodes : &HashSet<Rc<RefCell<Node>>>, path : &Vec<Rc<RefCell<Node>>>) -> bool {

    if start_node.as_ptr() == dest_node.as_ptr() {
        return true;
    }
    if visited_nodes.contains(start_node) {
        
    }

    false
}

fn main() {
    let mut g : Graph = new_graph();
    let n1 = new_node(&mut g, 1);
    let n2 = new_node(&mut g, 2);
    let n3 = new_node(&mut g, 3);
    let n4 = new_node(&mut g, 4);
    let n5 = new_node(&mut g, 5);
    let n6 = new_node(&mut g, 6);

    add_edge(&mut g, n1.clone(), n2.clone(), 1);
    add_edge(&mut g, n2.clone(), n3.clone(), 2);
    add_edge(&mut g, n3.clone(), n4.clone(), 3);
    add_edge(&mut g, n3.clone(), n5.clone(), 4);
    add_edge(&mut g, n4.clone(), n6.clone(), 5);
    add_edge(&mut g, n3.clone(), n1.clone(), 6);
    add_edge(&mut g, n1.clone(), n5.clone(), 7);
    add_edge(&mut g, n2.clone(), n5.clone(), 8);
    add_edge(&mut g, n5.clone(), n2.clone(), 9);
    add_edge(&mut g, n6.clone(), n4.clone(), 10);
    print_nodes(&g);


    //let result = find_path(&g, n1, n6);
    //println!("Result: {}", result);

}
