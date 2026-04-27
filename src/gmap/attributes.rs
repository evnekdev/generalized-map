// gmap::gmap.rs


use std::fmt::{Debug};
use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefMut, RefCell};
use std::any::{TypeId};
use std::collections::{HashMap};

use crate::index::{DartIndex, AttributeIndex, ATTRIBUTE_INDEX_NULL};
use crate::attribute::{AttributeContainer};
use crate::index_allocator::{IndexAllocator};
use super::{NGMap};

/// Configuration and management of attribute storage.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	/// Allocates one of *NA* slots for attribute storage
	///
	/// # Arguments
	///
	/// * `idim` - cell dimension for which the storage is allocated
	///
	/// * `onmerge` - a function defining a *merge* operation on attributes which is performed in case two i-cells are merged into one
	///
	/// * `onsplit` - a function defining a *split* operation on an attribute in case an i-cell is split into two
	///
	/// # Returns
	///
	/// `true` if allocation was successful;
	///
	/// `false` if the allocation for the given dimension `idim` already exists or the are no more available allocation slots (the maximum number is *NA*)
	pub fn reserve_attribute_container<T: Debug + Clone + 'static>(&mut self, idim: usize, onmerge: Box<dyn Fn(T,T)->T>, onsplit: Box<dyn Fn(T)->(T,T)>)->bool{
		assert!(idim < self.ndims);
		if self.attribute_container_indices.len() >= NA {
			return false;
		}
		if self.attribute_container_indices.get(&idim).is_some(){return false;}
		match self.attribute_containers.iter().position(|emb| emb.is_none()){
			Some(idx) => {
				self.attribute_container_indices.insert(idim, idx);
				self.attribute_containers[idx] = Some(RefCell::new(AttributeContainer::new::<T>(onmerge,onsplit)));
				self.attribute_index_allocators[idx] = Some(IndexAllocator::new(0).into());
				let darts: Vec<DartIndex> = self.iter_dart_per_cell(idim).collect();
				for dart in darts.iter(){
					let index : AttributeIndex = self.reserve_attribute_index(idim).unwrap();
					self.set_attribute_index_to_cell(dart, index, idim);
				}
				return true;
			}
			None => {return false;}
		}
	}
	
	/// Returns [`true`] if an attribute for `idim` dimension has been allocated using [`Self::reserve_attribute_container`].
	pub fn has_attribute_container(&self, idim: usize)->bool{
		return self.attribute_container_indices.get(&idim).is_some();
	}
	
	/// Returns a reference to the attribute container, wrapped in an [`Option`]
	pub fn get_attribute_container(&self, idim: usize)->Option<Ref<'_, AttributeContainer>>{
		return Some(self.attribute_containers[*self.attribute_container_indices.get(&idim)?].as_ref()?.borrow());
	}
	/// Returns a mutable reference to the attribute container, wrapped in an [`Option`]
	pub fn get_attribute_container_mut(&self, idim: usize)->Option<RefMut<'_, AttributeContainer>>{
		return Some(self.attribute_containers[*self.attribute_container_indices.get(&idim)?].as_ref()?.borrow_mut());
	}
	
	/// Returns `true` if there is an attribute stored for i-cell to which `dart` belongs
	pub fn has_attribute_for(&self, dart: &DartIndex, idim: usize)->bool{
		return self.get_attribute_container(idim).is_some_and(|emb| emb.has_data(&self.get_attribute_index(dart, idim)));
	}
	
	/// Deallocates a slot occupied by an attribute container and drops its
	pub fn free_attribute_container(&mut self, idim: usize){
		assert!(idim < self.ndims);
		match self.attribute_container_indices.get(&idim){
			Some(idx) => {
				self.attribute_containers[*idx] = None;
				self.attribute_index_allocators[*idx] = None;
				let darts: Vec<DartIndex> = self.iter_all().collect();
				for dart in darts.iter(){
					self.darts[**dart].set_attribute_index(AttributeIndex::null(),idim);
				}
			}
			None => {
				// pass
			}
		}
	}
	
	/// Deallocates all slots for attribute containers and drops them
	pub fn free_attribute_containers(&mut self){
		for k in 0..N {
			self.free_attribute_container(k);
		}
	}
	
	/// Returns the index in the attribute container corresponding to the i-cell of the `dart`.
	/// 
	/// If there is no attribute stored or the storage is not allocated, returns [`crate::index::AttributeIndex::null()`]
	pub fn get_attribute_index(&self, dart: &DartIndex, idim: usize)->AttributeIndex{
		match self.attribute_container_indices.get(&idim) {
			Some(idx) => {
				return self.darts[**dart].get_attribute_index(*idx);
			}
			None => {return AttributeIndex::null();}
		}
	}
	
	
	pub(super) fn set_attribute_index_to_cell(&mut self, dart: &DartIndex, index: AttributeIndex, idim: usize)->bool{
		match self.attribute_container_indices.get(&idim){
			Some(idx) => {
				let vec : Vec<DartIndex> = self.iter_darts_in_cell(*dart, idim).collect();
				for di in vec.into_iter() {
					self.darts[*di].set_attribute_index(index, *idx);
				}
				return true;
			}
			None => {
				return false;
			}
		}
	}
	
	pub(super) fn reserve_attribute_index(&self, idim: usize)->Option<AttributeIndex>{
		let idx = self.attribute_container_indices.get(&idim)?;
		return Some(self.attribute_index_allocators[*idx].as_ref()?.borrow_mut().reserve_index().into());
	}
	
	pub(super) fn merge_attributes(&mut self, dart1: &DartIndex, dart2: &DartIndex, idim: usize)->AttributeIndex{
		let index: AttributeIndex;
		match self.get_attribute_container_mut(idim) {
			Some(mut emb) => {
				let index1 = self.get_attribute_index(dart1, idim);
				let index2 = self.get_attribute_index(dart2, idim);
				index = self.reserve_attribute_index(idim).unwrap();
				emb.merge_data((index1, index2), index);
			}
			None => {return AttributeIndex::null();}
		}
		self.set_attribute_index_to_cell(dart1, index, idim);
		return index;
	}
	
	pub(super) fn split_attributes(&mut self, dart1: &DartIndex, dart2: &DartIndex, idim: usize)->(AttributeIndex, AttributeIndex){
		let index1: AttributeIndex;
		let index2: AttributeIndex;
		match self.get_attribute_container_mut(idim){
			Some(mut emb) => {
				let index = self.get_attribute_index(dart1, idim);
				index1 = self.reserve_attribute_index(idim).unwrap();
				index2 = self.reserve_attribute_index(idim).unwrap();
				emb.split_data(index, (index1,index2));
			}
			None => {return (AttributeIndex::null(), AttributeIndex::null());}
		}
		self.set_attribute_index_to_cell(dart1, index1, idim);
		self.set_attribute_index_to_cell(dart2, index2, idim);
		return (index1, index2);
	}
	
	/// Stores data as an attribute of the i-cell, to which `dart` belongs
	pub fn set_attribute_to_dart<T: Debug + Clone + 'static>(&self, dart: &DartIndex, data: T, idim: usize){
		
		match self.get_attribute_container_mut(idim) {
			Some(mut emb) => {
				emb.set_data(self.get_attribute_index(dart, idim), data);
			}
			None => {}
		}
	}
	
	pub fn remove_attribute_from_dart(&self, dart: &DartIndex, idim: usize){
		match self.get_attribute_container_mut(idim){
			Some(mut emb) => {
				emb.remove_data(self.get_attribute_index(dart, idim));
			}
			None => {}
		}
	}
	
	/// Retrieves stored attribute data, if any, from the i-cell to which `dart` belongs
	pub fn get_attribute_for_dart<'a, T: Debug + Clone + 'static>(&'a self, dart: &DartIndex, idim: usize)->Option<Ref<'a, T>>{
		let idx = self.attribute_container_indices.get(&idim)?;
		let emb : Ref<AttributeContainer> = self.get_attribute_container(idim)?;
		let index = self.darts[**dart].get_attribute_index(*idx);
		if !emb.has_data(&index){return None;}
		return Some(Ref::map(emb, |val| val.get_data(&index).unwrap()));
	}
	
	pub fn has_attribute_collisions<const N1: usize, const NA1: usize, const NL1: usize>(&self, other: &NGMap<N1,NA1,NL1>)->bool{
		let mut collisions: HashMap<usize, TypeId> = HashMap::new();
		for k in 0..N {
			let _ = self.get_attribute_container(k).map(|cnt| collisions.insert(k, cnt.get_key_type_id()));
		}
		for k in 0..N1 {
			if other.get_attribute_container(k).is_some_and(|cnt| collisions.get(&k) == Some(&cnt.get_key_type_id())){return true;}
		}
		return false;
	}
}
