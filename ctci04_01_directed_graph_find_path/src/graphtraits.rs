use std::fmt::Display;

pub type NodeId = usize;
pub type EdgeId = usize;

pub struct EdgeTriplet(pub NodeId, pub NodeId, pub i32);

pub type Path = Vec<EdgeId>;

pub trait GraphCrud<T> {
    fn new() -> Self
    where
        Self: Sized;
    fn new_node(&mut self, val: T) -> NodeId;
    fn set_node(&mut self, val: T, id: NodeId) -> NodeId;
    fn del_node(&mut self, nodeid: NodeId) -> bool;
    fn get_node_val(&self, nodeid: NodeId) -> Option<&T>;
    fn set_node_val(&mut self, nodeid: NodeId, val: T);

    fn new_edge(&mut self, from: NodeId, to: NodeId, weight: i32) -> EdgeId;
    fn del_edge(&mut self, edge: EdgeId) -> bool;
    fn get_edge(&self, edge: EdgeId) -> Option<EdgeTriplet>;
    fn set_edge(&mut self, edge: EdgeId, edge_data: EdgeTriplet);

    fn find_edges_from(&self, from: EdgeId) -> Option<Vec<EdgeId>>;
    fn find_edges_to(&self, to: EdgeId) -> Option<Vec<EdgeId>>;
}

pub trait GraphDisplay<T: Display> {
    fn to_string(&self) -> String;
    fn node_to_string(&self, node_id: NodeId) -> Option<String>;
    fn edge_to_string(&self, edge_id: EdgeId) -> Option<String>;
    fn path_to_string(&self, path: &Path) -> Option<String>;
}

pub trait GraphAlgo<T> {
    fn path_exists(&self, from: NodeId, to: NodeId) -> bool;
    fn shortest_path(&self, from: NodeId, to: NodeId) -> Option<Path>;
    fn all_paths(&self, from: NodeId, to: NodeId) -> Option<Vec<Path>>;
}
