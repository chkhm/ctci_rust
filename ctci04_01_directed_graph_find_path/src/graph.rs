use std::collections::HashMap;

use crate::graphtraits::{EdgeId, EdgeTriplet, GraphCrud, GraphDisplay, NodeId, Path};

pub struct Graph<T> {
    last_id: NodeId,
    nodes: HashMap<NodeId, Node<T>>,
    edges: HashMap<EdgeId, Edge>,
}

struct Node<T> {
    value: T,
}

struct Edge {
    from: NodeId,
    to: NodeId,
    weight: i32,
}

impl<T> GraphCrud<T> for Graph<T> {
    fn new() -> Self {
        Graph {
            last_id: 0,
            nodes: HashMap::new(),
            edges: HashMap::new(),
        }
    }

    fn new_node(&mut self, value: T) -> NodeId {
        let n = Node { value };
        self.last_id += 1;
        self.nodes.insert(self.last_id, n);
        self.last_id
    }

    fn set_node(&mut self, value: T, id: NodeId) -> NodeId {
        let n = Node { value };
        self.nodes.insert(id, n); // will overwrite old value thee was one
        id
    }

    fn del_node(&mut self, nodeid: NodeId) -> bool {
        if self.nodes.contains_key(&nodeid) {
            self.nodes.remove(&nodeid);
            let mut edges_to_be_removed: Vec<EdgeId> = Vec::new();
            for (edge_id, edge) in self.edges.iter() {
                if edge.from == nodeid || edge.to == nodeid {
                    edges_to_be_removed.push(*edge_id);
                }
            }
            edges_to_be_removed.iter().for_each(|edge_id| {
                self.edges.remove(edge_id);
            });
            return true;
        }
        false
    }

    fn get_node_val(&self, nodeid: NodeId) -> Option<&T> {
        let x = self.nodes.get(&nodeid);
        match x {
            Some(node) => Some(&node.value),
            None => None,
        }
    }

    fn set_node_val(&mut self, nodeid: NodeId, val: T) {
        self.nodes.entry(nodeid).and_modify(|node| node.value = val);
    }

    fn new_edge(&mut self, from: NodeId, to: NodeId, weight: i32) -> EdgeId {
        self.last_id += 1;
        self.edges.insert(self.last_id, Edge { from, to, weight });
        self.last_id
    }

    fn del_edge(&mut self, edgeid: EdgeId) -> bool {
        self.edges.remove(&edgeid).is_some()
    }

    fn get_edge(&self, edgeid: EdgeId) -> Option<EdgeTriplet> {
        let entry = self.edges.get(&edgeid);
        entry.map(|e| EdgeTriplet(e.from, e.to, e.weight))
    }

    fn set_edge(&mut self, edgeid: EdgeId, edge_data: EdgeTriplet) {
        self.edges.entry(edgeid).and_modify(|e| {
            e.from = edge_data.0;
            e.to = edge_data.1;
            e.weight = edge_data.2
        });
    }

    fn find_edges_from(&self, from: NodeId) -> Option<Vec<EdgeId>> {
        if self.nodes.contains_key(&from) {
            let out_edge: Vec<EdgeId> = self
                .edges
                .iter()
                .filter(|e| e.1.from == from)
                .map(|e| *e.0)
                .collect();
            return Some(out_edge);
        }
        None
    }

    fn find_edges_to(&self, to: EdgeId) -> Option<Vec<EdgeId>> {
        if self.nodes.contains_key(&to) {
            let out_edge: Vec<EdgeId> = self
                .edges
                .iter()
                .filter(|e| e.1.to == to)
                .map(|e| *e.0)
                .collect();
            return Some(out_edge);
        }
        None
    }
}

impl<T: std::fmt::Display> GraphDisplay<T> for Graph<T> {
    fn to_string(&self) -> String {
        let mut nodes_string = String::new();
        for (idx, n) in self.nodes.iter() {
            nodes_string.push_str(&format!(
                "{{\"nodeid\": \"{}\", \"value\": \"{}\"}},",
                idx, n.value
            ));
        }
        let mut edges_string = String::new();
        for (idx, e) in self.edges.iter() {
            edges_string.push_str(&format!(
                "{{\"edgeid\": \"{}\", \"from\": \"{}\", \"to\": \"{}\", \"weight\": \"{}\"",
                idx, e.from, e.to, e.weight
            ));
        }

        let result = format!(
            "{{ \"nodes\": {}, \"edges\": {} }}",
            nodes_string, edges_string
        );
        result
    }

    fn node_to_string(&self, node_id: NodeId) -> Option<String> {
        let node_opt = self.nodes.get(&node_id);
        node_opt.map(|node| format!("{{ \"nodeid\": {}, \"value\": {} }}", node_id, node.value))
    }

    fn edge_to_string(&self, edge_id: EdgeId) -> Option<String> {
        let edge_opt = self.edges.get(&edge_id);
        edge_opt.map(|edge| {
            format!(
                "{{ \"edge_id\": {}, \"from\": \"{}\", \"to\": \"{}\", \"weight\": {} }}",
                edge_id, edge.from, edge.to, edge.weight
            )
        })
    }

    fn path_to_string(&self, path: &Path) -> Option<String> {
        let mut result = String::new();
        for n in path.iter() {
            let s = self.node_to_string(*n);
            match s {
                Some(s) => result.push_str(&s),
                None => return None,
            }
        }
        Some(result)
    }
}
