//use crate::*;

use std::collections::{HashMap};
use std::fs::{File,OpenOptions};
use std::io::{Write};

use std::any::{Any};
use downcast_rs::{Downcast, impl_downcast};

use generalized_map::{NGMap};
use generalized_map::index::{DartIndex};
use generalized_map::inspect::{DartInspector};
use generalized_map::output::{list_nodes_and_edges, write_nodes_and_edges};
use generalized_map::index_allocator::{IndexAllocator};

pub fn create_2glued_rings()->NGMap<3,1,0>{
	let mut map = NGMap::<3,1,0>::new(3);
	let darts1 = map.create_ring(8);
	let darts2 = map.create_ring(4);
	type cell_type_2 = (f64,f64);
	map.reserve_attribute_container::<cell_type_2>(2, Box::new(|v1: cell_type_2, v2: cell_type_2| ((v1.0+v2.0)*0.5, (v1.1+v2.1)*0.5)), Box::new(|v: cell_type_2| (v.clone(),v)));
	map.set_attribute_to_dart(&darts1[0].0, (0.5, 0.5), 2);
	map.sew(&darts1[0].0, &darts2[0].0, 2);
	return map;
}

pub fn iterate_cells<const N: usize, const NA: usize, const NL: usize>(map: &NGMap<N,NA,NL>){
	map.iter_dart_per_cell(0).for_each(|d| print!("d[0] = {} ", &d));
	println!("");
	map.iter_dart_per_cell(1).for_each(|d| print!("d[1] = {} , data = {:?} ", &d, map.get_attribute_for_dart::<(f64,f64)>(&d, 2)));
	println!("");
	map.iter_dart_per_cell(2).for_each(|d| print!("d[2] = {} , data = {:?} ", &d, map.get_attribute_for_dart::<(f64,f64)>(&d, 2)));
	println!("");
}

pub fn iterate_incident<const N: usize, const NA: usize, const NL: usize>(map: &NGMap<N,NA,NL>){
	let dp = DartIndex::from(0);
	map.iter_incident(dp, 1, 0).for_each(|d| print!("d[0] = {} ", &d));
	println!("");
	map.iter_incident(dp, 0, 2).for_each(|d| print!("d[1] = {} ", &d));
	println!("");
	map.iter_incident(dp, 1, 2).for_each(|d| print!("d[2] = {} ", &d));
	println!("");
}

pub fn iterate_adjacent<const N: usize, const NA: usize, const NL: usize>(map: &NGMap<N,NA,NL>){
	let dp = DartIndex::from(0);
	map.iter_adjacent(dp, 0).for_each(|d| print!("d[0] = {} ", &d));
	println!("");
	map.iter_adjacent(dp, 1).for_each(|d| print!("d[1] = {} ", &d));
	println!("");
	map.iter_adjacent(dp, 2).for_each(|d| print!("d[2] = {} ", &d));
	println!("");
}

pub fn map_iterate()->NGMap<3,1,0>{
	let map = create_2glued_rings();
	iterate_cells(&map);
	iterate_incident(&map);
	iterate_adjacent(&map);
	return map;
}

pub fn remove_vertex()->NGMap<3,0,0>{
	let mut map = NGMap::<3,0,0>::new(3);
	let darts = map.create_ring(6);
	let d0 = darts[0].1;
	let d1 = darts[1].1;
	map.iter_darts_in_cell(d0, 2).for_each(|d| print!("{}, ", &d));
	println!("removable0 {}, removable1 {}, removable2 {}", map.is_removable(&d1, 0), map.is_removable(&d1, 1), map.is_removable(&d1, 2));
	map.remove_cell(&d1,0);
	map.iter_darts_in_cell(d0, 2).for_each(|d| print!("{}, ", &d));
	return map;
}

pub fn insert_vertex()->NGMap<3,0,0>{
	let mut map = NGMap::<3,0,0>::new(3);
	let darts = map.create_ring(3);
	let d0 = darts[0].1;
	let d1 = darts[1].0;
	map.iter_darts_in_cell(d0, 2).for_each(|d| print!("{}, ", &d));
	let (e0, e1) = map.create_vertex();
	let assoc : HashMap<DartIndex,DartIndex> = HashMap::from([(e0,d0),(e1,d1)]);
	println!("insertable0 {}, insertable1 {}, insertable2 {}", map.is_insertable(&d0, &e0, 0, &assoc), map.is_insertable(&d0, &e0, 1, &assoc), map.is_insertable(&d0, &e0, 2, &assoc));
	map.insert(&d0, &e0, 0, &assoc);
	map.iter_darts_in_cell(d0, 2).for_each(|d| print!("{}, ", &d));
	return map;
}

pub fn print_nodes_and_edges<const N: usize, const NA: usize, const NL: usize>(map: &NGMap<N,NA,NL>){
	let (nodes, edges) = list_nodes_and_edges(map);
	println!("nodes = {:?}", &nodes);
	println!("edges = {:?}", &edges);
	let filename = r"c:\_WORK\Code\Rust\workspace\gmap\visual\nodes_and_edges.dat";
	write_nodes_and_edges(filename, &nodes, &edges);
}

pub fn create_shape()->NGMap<4,0,0>{
	let mut map = NGMap::<4,0,0>::new(3);
	//map.create_star(3, true);
	//map.create_vertex_pizza(20);
	map.create_ring(4);
	//map.print();
	return map;
}

pub fn create_shape_with_attributes()->NGMap<3,1,0>{
	let mut map = NGMap::<3,1,0>::new(2);
	let nedges = 3;
	let vec : Vec<f64> = (0..nedges+1).into_iter().map(|val| val as f64 *0.5).collect();
	map.create_chain_with_vertex_attributes(&mut vec.into_iter());
	map.print();
	map.iter_dart_per_cell(0).for_each(|d| print!("{}:{:?} ", &d, map.get_attribute_for_dart::<f64>(&d, 0)));
	return map;
}

pub fn extrude()->NGMap<4,0,0>{
	let mut map = NGMap::<4,0,0>::new(2);
	let vec1 = map.create_chain(3);
	let vec2 = map.create_chain(3);
	//println!("vec1 = {:?}", &vec1);
	//println!("vec2 = {:?}", &vec2);
	//let vec3 = map.create_chain(2);
	let map1 = map.extrude(&vec1[0].0, &vec2[0].0, 1).unwrap();
	//map.extrude(&d, &vec3[0].0);
	//map.remove_component(&vec1[0].0);
	//map.remove_component(&vec2[0].0);
	//map.remove_component(&vec3[0].0);
	//let mut map1 = NGMap::<4,0,0>::new();
	//map1.copy_from(&map);
	map1.print();
	return map1;
}

pub fn extrude_mesh()->NGMap<3,1,0>{
	let mut map = NGMap::<3,1,0>::new(2);
	let coords1 : Vec<f64> = vec![-1.0, -0.5, 0.0, 0.5, 1.0];
	let coords2 : Vec<f64> = vec![-1.0, 0.0, 1.0];
	let vec1 = map.create_chain_with_vertex_attributes(&mut coords1.into_iter());
	let vec2 = map.create_chain_with_vertex_attributes(&mut coords2.into_iter());
	map.iter_dart_per_cell(0).for_each(|d| print!("{}:{:?} ", &d, map.get_attribute_for_dart::<f64>(&d, 0)));
	map.print();
	let map1 = map.extrude_with_vertex_attributes::<f64,f64>(&vec1[0].0, &vec2[0].0, 1).unwrap();
	map1.print();
	for (idx, dart) in map1.iter_dart_per_cell(0).enumerate(){
		let data = map1.get_attribute_for_dart::<(f64,f64)>(&dart, 0);
		println!("idx = {:?}, data = {:?}", &idx, &data);
	}
	return map1;
}

pub fn chamfer()->NGMap<4,0,0>{
	let mut map = NGMap::<4,0,0>::new(4);
	let vec1 = map.create_ring(4);
	let vec2 = map.create_chain(1);
	let mut map1 = map.extrude(&vec1[0].0, &vec2[0].0, 2).unwrap();
	let darts : Vec<DartIndex> = map1.iter_dart_per_cell(0).collect();
	for dart in darts.iter(){
		map1.chamfer(dart, 0, 3);
	}
	map1.print();
	return map1;
}

pub fn allocate_index(){
	let mut allocator = IndexAllocator::new(0);
	let i1 = allocator.reserve_index();
	let i2 = allocator.reserve_index();
	let i3 = allocator.reserve_index();
	let i4 = allocator.reserve_index();
	let i5 = allocator.reserve_index();
	println!("i1 = {}, i2 = {}, i3 = {}, i4 = {}, i5 = {}", &i1, &i2, &i3, &i4, &i5);
	allocator.free_index(i4);
	//allocator.free_index(i3);
	allocator.free_index(i2);
	println!("allocator = {:?}", &allocator);
	let i6 = allocator.reserve_index();
	let i7 = allocator.reserve_index();
	let i8_= allocator.reserve_index();
	let i9 = allocator.reserve_index();
	println!("allocator = {:?}", &allocator);
	println!("i6 = {}, i7 = {}, i8 = {}, i9 = {}", &i6, &i7, &i8_, &i9);
}

pub fn main(){
	//let map = map_iterate();
	//let map = remove_vertex();
	//let map = insert_vertex();
	//let map = create_2glued_rings();
	//let map = create_shape();
	//let map = extrude();
	//let map = extrude_mesh();
	let map = chamfer();
	let mut map = map.reallocate::<6,1,1>();
	map.change_dim(2);
	map.change_dim(5);
	//let map = create_shape_with_attributes();
	//allocate_index();
	//print_nodes_and_edges(&map);
	//println!("{}", &map.to_json());
	let mut file = OpenOptions::new().write(true).create(true).open(r"c:\_WORK\Code\Rust\workspace\gmap\visual\map.json").unwrap();
	file.write(&map.to_json().as_bytes());
}