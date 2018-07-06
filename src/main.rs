extern crate petgraph;

use std::error::Error;
use std::io::prelude::*;
use std::fs::File;

use petgraph::graph::Graph;
use petgraph::dot::{Dot, Config};

fn main() {
    // Create our nodes in the map
    let mut sol_graph = Graph::<&str, f64, petgraph::Undirected>::new_undirected();
    let earth = sol_graph.add_node("Earth");
    let leo = sol_graph.add_node("LEO");
    let gto = sol_graph.add_node("GTO");
    let geo = sol_graph.add_node("GEO");
    let lto = sol_graph.add_node("LTO");
    let lunar_escape = sol_graph.add_node("Lunary Capture/Escape");
    let llo = sol_graph.add_node("Low Lunar Orbit");
    let luna = sol_graph.add_node("Luna");
    let earth_escape = sol_graph.add_node("Earth Capture/Escape");

    // Connect the nodes
    sol_graph.add_edge(earth, leo, 9.4);
    sol_graph.add_edge(earth, gto, 2.44);

    sol_graph.add_edge(gto, geo, 1.47);
    sol_graph.add_edge(gto, lto, 0.68);

    sol_graph.add_edge(lto, lunar_escape, 0.14);
    sol_graph.add_edge(lto, earth_escape, 0.009);

    sol_graph.add_edge(lunar_escape, llo, 0.68);
    sol_graph.add_edge(llo, luna, 1.73);

    let mut dot = Dot::with_config(&sol_graph, &[Config::EdgeNoLabel]);

    let dotfile_name = "solmap.dot";
    let mut dotfile = match File::create(dotfile_name) {
        Err(why) => panic!("couldn't create {}: {}",
                           dotfile_name,
                           why.description()),
        Ok(file) => file,
    };
    let dot_data = format!("{:?}", dot);

    match dotfile.write_all(dot_data.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", dotfile_name, why.description()),
        Ok(_) => println!("success!"),
    };
}
