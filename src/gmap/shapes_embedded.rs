// gmap::gmap.rs

use std::fmt::{Debug};
use std::collections::{HashMap};

use crate::index::{DartIndex};
use crate::attribute::{onmerge_default, onsplit_default};
use super::{NGMap};

/// Various topological primitives with additional attribute loads.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	pub fn create_edge_with_attribute<T: Debug + Clone + 'static>(&mut self, data: T)->(DartIndex,DartIndex){
		if !self.has_attribute_container(1){
			self.reserve_attribute_container::<T>(1, Box::new(onmerge_default), Box::new(onsplit_default));
		}
		let (d1, d2) = self.create_edge();
		self.set_attribute_to_dart::<T>(&d1, data, 1);
		return (d1,d2);
	}
	
	pub fn create_vertex_with_attribute<T: Debug + Clone + 'static>(&mut self, data: T)->(DartIndex,DartIndex){
		if !self.has_attribute_container(0){
			self.reserve_attribute_container::<T>(0, Box::new(onmerge_default), Box::new(onsplit_default));
		}
		let (d1,d2) = self.create_vertex();
		self.set_attribute_to_dart::<T>(&d1, data, 0);
		return (d1, d2);
	}
	
	pub fn create_chain_with_vertex_attributes<T: Debug + Clone + 'static>(&mut self, iter: &mut dyn ExactSizeIterator<Item=T>)->Vec<(DartIndex,DartIndex)>
	{
		if !self.has_attribute_container(0){
			self.reserve_attribute_container::<T>(0, Box::new(onmerge_default), Box::new(onsplit_default));
		}
		let vec = self.create_chain(iter.len()-1);
		for k in 0..vec.len(){
			let pair = &vec[k];
			let data = iter.next().unwrap();
			self.set_attribute_to_dart::<T>(&pair.0, data, 0);
		}
		self.set_attribute_to_dart::<T>(&vec[vec.len()-1].1, iter.next().unwrap(), 0);
		return vec;
	}
	
	pub fn create_chain_with_edge_attributes<T: Debug + Clone + 'static>(&mut self, iter: &mut dyn ExactSizeIterator<Item=T>)->Vec<(DartIndex,DartIndex)>
	{
		if !self.has_attribute_container(1){
			self.reserve_attribute_container::<T>(1, Box::new(onmerge_default), Box::new(onsplit_default));
		}
		let vec = self.create_chain(iter.len());
		for (pair, data) in vec.iter().zip(iter){
			self.set_attribute_to_dart::<T>(&pair.1, data, 1);
		}
		return vec;
	}
	
	pub fn create_chain_with_vertex_edge_attributes<T1,T2>(&mut self, iter0: &mut dyn ExactSizeIterator<Item=T1>, iter1: &mut dyn ExactSizeIterator<Item=T2>)->Vec<(DartIndex,DartIndex)>
	where T1: Debug + Clone + 'static, T2: Debug + Clone + 'static,
	{
		todo!();
	}
	
	pub fn create_ring_with_vertex_attributes<T>(&mut self, iter: &mut dyn ExactSizeIterator<Item=T>)->Vec<(DartIndex,DartIndex)>
	where T: Debug + Clone + 'static,
	{
		if !self.has_attribute_container(0){
			self.reserve_attribute_container::<T>(0, Box::new(onmerge_default), Box::new(onsplit_default));
		}
		let vec = self.create_ring(iter.len());
		for (pair, data) in vec.iter().zip(iter){
			self.set_attribute_to_dart::<T>(&pair.1, data, 0);
		}
		return vec;
	}
	
	pub fn create_ring_with_edge_attributes<T>(&mut self, iter: &mut dyn ExactSizeIterator<Item=T>)->Vec<(DartIndex,DartIndex)>
	where T: Debug + Clone + 'static,
	{
		if !self.has_attribute_container(1){
			self.reserve_attribute_container::<T>(1, Box::new(onmerge_default), Box::new(onsplit_default));
		}
		let vec = self.create_ring(iter.len());
		for (pair, data) in vec.iter().zip(iter){
			self.set_attribute_to_dart::<T>(&pair.1, data, 1);
		}
		return vec;
	}
	
	pub fn create_ring_with_vertex_edge_attributes<T0,T1>(&mut self, iter0: &mut dyn ExactSizeIterator<Item=T0>, iter1: &mut dyn ExactSizeIterator<Item=T1>)->Vec<(DartIndex,DartIndex)>
	where T0: Debug + Clone + 'static, T1: Debug + Clone + 'static,
	{	
		assert!(iter0.len() == iter1.len());
		if !self.has_attribute_container(0){
			self.reserve_attribute_container::<T0>(0, Box::new(onmerge_default), Box::new(onsplit_default));
		}
		if !self.has_attribute_container(1){
			self.reserve_attribute_container::<T1>(1, Box::new(onmerge_default), Box::new(onsplit_default));
		}
		let vec = self.create_ring(iter0.len());
		for (pair, (data0,data1)) in vec.iter().zip(iter0.zip(iter1)){
			self.set_attribute_to_dart::<T0>(&pair.1, data0, 0);
			self.set_attribute_to_dart::<T1>(&pair.1, data1, 0);
		}
		return vec;
	}
	
}
