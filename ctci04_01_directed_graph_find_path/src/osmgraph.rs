use std::{collections::HashMap, fs::File, io::BufReader};

use xml::{EventReader, attribute::OwnedAttribute, reader::XmlEvent};

use crate::{graph::Graph, graphtraits::GraphCrud};

pub struct OsmNode {
    pub id: usize,
    pub lat: f64,
    pub lon: f64,
    pub version: i32,
}

pub fn create_osm_graph() -> Graph<OsmNode> {
    Graph::new()
}

/**
 * This function parses until the EndDocument for this element is found
 */
fn parse_until_end_element(parser: &mut EventReader<BufReader<File>>, name: &str) {
    let mut keep_parsing = true;
    while keep_parsing {
        let event = parser.next();
        match event {
            Ok(XmlEvent::EndElement { name: end_name }) => {
                if name != end_name.local_name {
                    println!(
                        "strange unfit of startname and endname {} - {}",
                        name, end_name
                    );
                }
                keep_parsing = false;
            }
            Ok(XmlEvent::StartElement {
                name,
                attributes: _,
                namespace: _,
            }) => {
                parse_until_end_element(parser, &name.local_name);
            }
            Ok(XmlEvent::EndDocument) => keep_parsing = false,
            _ => {
                keep_parsing = true;
            }
        }
    }
}

fn parse_osm_node_event(
    parser: &mut EventReader<BufReader<File>>,
    g: &mut Graph<OsmNode>,
    attributes: &[OwnedAttribute],
) {
    let attr: HashMap<String, String> = attributes
        .iter()
        .map(|a| (a.name.local_name.clone(), a.value.clone()))
        .collect(); //::<HashMap<_,_>>();
    let osmnode = OsmNode {
        id: attr["id"].parse().unwrap(),
        lat: attr["lat"].parse().unwrap(),
        lon: attr["lon"].parse().unwrap(),
        version: attr["version"].parse().unwrap(),
    };
    let id = osmnode.id;
    g.set_node(osmnode, id);
    parse_until_end_element(parser, "node"); // jump over tags
}

fn parse_osm_nd_event(
    parser: &mut EventReader<BufReader<File>>,
    attributes: &[OwnedAttribute],
) -> Option<usize> {
    parse_until_end_element(parser, "nd");
    let id_attr = attributes.iter().find(|a| a.name.local_name == "ref")?;
    let x = id_attr.value.parse();
    x.ok()
}

fn parse_osm_way_event(parser: &mut EventReader<BufReader<File>>, g: &mut Graph<OsmNode>) {
    let mut nd_vec = Vec::new();
    let mut keep_parsing = true;
    while keep_parsing {
        let event = parser.next();
        match event {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            }) => {
                if name.local_name == "nd" {
                    let id = parse_osm_nd_event(parser, &attributes);
                    if let Some(num) = id {
                        nd_vec.push(num);
                    }
                } else {
                    // most likely a tag, we ignore any other tag, we are only interested in "nd"
                    parse_until_end_element(parser, &name.local_name);
                }
            }
            Ok(XmlEvent::EndElement { name: _ }) => {
                keep_parsing = false;
            }
            Ok(XmlEvent::EndDocument) => {
                println!("unexpected end docu");
                keep_parsing = false;
            }
            _ => {}
        }
    }
    // now the vec has all the waypoints. Let's add them ass edges
    if nd_vec.len() < 2 {
        return;
    }
    let mut it = nd_vec.iter();
    let mut from = *(it.next().unwrap());
    for nd in it {
        let weight = 1;
        g.new_edge(from, *nd, weight);
        from = *nd;
    }
}

fn parse_osm_body(parser: &mut EventReader<BufReader<File>>, g: &mut Graph<OsmNode>) {
    let mut keep_parsing = true;
    while keep_parsing {
        let event = parser.next();
        match event {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            }) => {
                if name.local_name == "node" {
                    parse_osm_node_event(parser, g, &attributes);
                } else if name.local_name == "way" {
                    parse_osm_way_event(parser, g);
                } else {
                    // ignore unkown element
                    parse_until_end_element(parser, &name.local_name);
                }
            }
            Ok(XmlEvent::EndDocument) => {
                keep_parsing = false;
            }
            _ => {}
        }
    }
}

// wait for osm start event, then read node and way events, all other ignore. If endosm is found then stop
pub fn parse_osm(parser: &mut EventReader<BufReader<File>>, g: &mut Graph<OsmNode>) {
    let mut keep_parsing = true;
    while keep_parsing {
        let event = parser.next();
        match event {
            Ok(XmlEvent::StartElement {
                name,
                attributes,
                namespace: _,
            }) => {
                if name.local_name == "osm" {
                    parse_osm_body(parser, g);
                } else {
                    // ignore unkown element
                    parse_until_end_element(parser, &name.local_name);
                }
            }
            Ok(XmlEvent::EndDocument) => {
                keep_parsing = false;
            }

            _ => {}
        }
    }
}
