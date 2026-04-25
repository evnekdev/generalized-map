// gmap::output.rs

//! Transforms a generalized map into a set of vertices and edges and outputs it for graph visualization purposes

use std::collections::{HashMap, BTreeSet, BTreeMap};
use std::fs::{File};
use std::io::{LineWriter, Write};

use crate::gmap::{NGMap};
use crate::index::{DartIndex};

// TODO where is something wrong with the algorithm here
pub fn list_nodes_and_edges<const N: usize, const NA: usize, const NL: usize>(map: &NGMap<N,NA,NL>)->(Vec<i32>, Vec<[i32;2]>){
	let mut dartnodes: HashMap<DartIndex,i32> = HashMap::new();
	
	for (node, dart0) in map.iter_dart_per_cell(0).enumerate(){
		for dart in map.iter_darts_in_cell(dart0, 0){
			dartnodes.insert(dart, node as i32);
		}
	}
	
	let mut nodes : BTreeSet<i32> = BTreeSet::new();
	for (_, node) in dartnodes.iter(){
		nodes.insert(*node);
	}
	let mut edges : BTreeMap<usize,[i32;2]> = BTreeMap::new();
	let mut count = -1;
	for (edge, dart) in map.iter_dart_per_cell(1).enumerate(){
		let mut ns : BTreeSet<i32> = BTreeSet::new();
		for d in map.iter_darts_in_cell(dart, 1){
			ns.insert(*dartnodes.get(&d).unwrap());
		}
		let ns : Vec<i32> = ns.into_iter().collect();
		if ns.len() == 1 {
		} else {
			edges.insert(edge, [ns[0],ns[1]]);
		}
	}
	return (nodes.into_iter().collect(), edges.into_values().collect());
}

pub fn write_nodes_and_edges(filepath: &str, nodes: &Vec<i32>, edges: &Vec<[i32;2]>)->std::io::Result<()>{
	let file = File::create(filepath)?;
	let mut file = LineWriter::new(file);
	file.write_all(&format!("{} {}\n", nodes.len(), edges.len()).as_bytes())?;
	for node in nodes.iter(){
		file.write_all(&format!("{}\n", *node).as_bytes())?;
	}
	for edge in edges.iter(){
		file.write_all(&format!("{} {}\n", edge[0], edge[1]).as_bytes())?;
	}
	file.flush()?;
	return Ok(());
}