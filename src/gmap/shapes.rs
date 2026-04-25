// gmap::gmap.rs

use std::collections::{HashMap};

use crate::index::{DartIndex};
use super::{NGMap};

/// Various simple topological primitives which can be used as building blocks for more complex structures.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	/// Creates two darts, alpha(*I*)-linked to each other.
	pub fn create_linked_pair(&mut self, idim: usize)->(DartIndex, DartIndex){
		assert!(idim < self.ndims);
		let dart1 = self.create_dart();
		let dart2 = self.create_dart();
		self.sew(&dart1, &dart2, idim);
		return (dart1, dart2);
	}
	
	/// Creates a pair of darts, alpha(0)-linked to each other (a geometrical edge).
	pub fn create_edge(&mut self)->(DartIndex, DartIndex){
		return self.create_linked_pair(0);
	}
	
	/// Creates a pair of darts, alpha(1)-linked to each other (a vertex with two open bonds).
	pub fn create_vertex(&mut self)->(DartIndex, DartIndex){
		return self.create_linked_pair(1);
	}
	
	/// Creates `nlinks` > 0 edges, joined together. Darts in this structure are alternately joined by alpha(0) and alpha(1) connections.
	pub fn create_chain(&mut self, nlinks: usize)->Vec<(DartIndex,DartIndex)>{
		assert!(nlinks > 0);
		let edges : Vec<(DartIndex, DartIndex)> = (0..nlinks).into_iter().map(|_| self.create_edge()).collect();
		for k in 0..nlinks-1 {
			let prev = k;
			let next = k+1;
			self.sew(&edges[prev].1, &edges[next].0, 1);
		}
		return edges;
	}
	
	/// A chain with its ends connected to each other.
	pub fn create_ring(&mut self, nlinks: usize)->Vec<(DartIndex,DartIndex)>{
		assert!(nlinks > 0);
		let vertices : Vec<(DartIndex, DartIndex)> = (0..nlinks).into_iter().map(|_| self.create_vertex()).collect();
		for k in 0..nlinks {
			let prev = k;
			let next = (k+1) % nlinks;
			self.sew(&vertices[prev].1, &vertices[next].0, 0);
		}
		return vertices;
	}

	pub fn create_star(&mut self, nrays: usize, with_ends: bool)->Vec<(DartIndex, DartIndex)>{
		let core : Vec<(DartIndex, DartIndex)> = (0..nrays).into_iter().map(|_| self.create_vertex()).collect();
		for k in 0..nrays {
			let prev = k;
			let next = (k+1) % nrays;
			let res = self.sew(&core[prev].1, &core[next].0, 2);
			//println!("sewing {}-{}, {}", &prev, &next, res);
		}
		//self.print();
		println!("core = {:?}", &core);
		if with_ends {
			let edges : Vec<(DartIndex, DartIndex)> = (0..nrays).into_iter().map(|_| self.create_linked_pair(2)).collect();
			println!("edges = {:?}", &edges);
			/*
			for ((c1,c2),(e1,e2)) in core.iter().zip(edges.iter()){
				self.sew(c1,e2,0);
			}
			*/
			for k in 0..nrays{
				self.sew(&core[k].0, &edges[k].1, 0);
			}
			return edges;
		}
		return core;
	}
	
	
	pub fn create_tetrahedron(&mut self)->DartIndex{
		let stars : Vec<Vec<(DartIndex,DartIndex)>> = (0..4).into_iter().map(|_| self.create_star(3, false)).collect();
		todo!();
	}
	
	pub fn create_vertex_pizza(&mut self, nslices: usize)->Vec<(DartIndex, DartIndex)>{
		let star = self.create_star(nslices, true);
		let ring = self.create_ring(nslices);
		self.print();
		for k in 0..nslices {
			/*
			let prev = (k + nslices-1) % nslices;
			let next = k;
			let assoc : HashMap<DartIndex,DartIndex> = HashMap::from([(star[prev].1, ring[next].0),(star[next].0, ring[next].1)]);
			*/
			let assoc : HashMap<DartIndex,DartIndex> = HashMap::from([(star[k].0,ring[k].0), (star[k].1,ring[k].1)]);
			//println!("assoc = {:?}, is_insertable = {}", &assoc, self.is_insertable(&ring[next].0, &star[prev].1, 1, &assoc));
			self.insert(&ring[k].0, &star[k].0, 1, &assoc);
		}
		return ring;
	}
}
