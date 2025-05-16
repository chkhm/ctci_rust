use std::{fs::File, io::BufReader};

use graphtraits::GraphCrud;
use osmgraph::{create_osm_graph, parse_osm};
use rcgraph::{Graph, add_edge, find_path, new_graph, new_node, print_nodes};
use xml::EventReader;

pub mod graph;
pub mod graphtraits;
pub mod idgraph;
pub mod osmgraph;
pub mod rcgraph;

fn main() -> std::io::Result<()> {
    let mut g: Graph = new_graph();
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

    let result = find_path(&g, n1, n6);
    println!("Result: {}", result);

    let file = File::open("./test.osm")?;
    let reader = BufReader::new(file);
    let mut reader = EventReader::new(reader);
    let mut osmgraph = create_osm_graph();
    parse_osm(&mut reader, &mut osmgraph);
    let n = osmgraph.get_node_val(8945281129).unwrap();
    println!(
        "node id: {}, lat: {}, lon: {}, version: {}",
        n.id, n.lat, n.lon, n.version
    );

    let e = osmgraph.get_edge(1).unwrap();
    println!("Edge 1: from: {}, to: {}, weight: {}", e.0, e.1, e.2);
    let e = osmgraph.get_edge(2).unwrap();
    println!("Edge 2: from: {}, to: {}, weight: {}", e.0, e.1, e.2);

    Ok(())
}
