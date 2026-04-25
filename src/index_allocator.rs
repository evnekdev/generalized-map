// gmap::index_allocator.rs

//! Utility guard controlling how indices in a storage vector are occupied and freed.

use std::collections::{LinkedList};
use std::fmt::{Debug};
use std::cmp::{Ordering};

use serde::{Serialize,Deserialize,ser::{Serializer},de::{Deserializer}};

#[derive(Debug,Serialize,Deserialize)]
struct AllocationNode{
	start: usize,
	count: usize,
}

impl PartialEq<usize> for AllocationNode {
	fn eq(&self, other: &usize)->bool{
		return other >= &self.start && other <= &(self.start + self.count);
	}
}

impl PartialOrd<usize> for AllocationNode {
	fn partial_cmp(&self, other: &usize)->Option<Ordering>{
		if other < &self.start{
			return Some(Ordering::Less);
		}
		if self.eq(other) {return Some(Ordering::Equal);}
		return Some(Ordering::Greater);
	}
}

/// Allocates and deallocates indices for storage in a vector. After initialization, allocated indices grow by one, if some of them are freed later on, the consequent allocations will occupy the space of the previously deallocated indices.
/// Internally, uses a [`LinkedList`] to store continuous intervals of deallocated indices. If the list is empty, a new index is produced by incrementing an internal counter by one.
#[derive(Debug,Serialize,Deserialize)]
pub struct IndexAllocator {
	count: usize,
	nodes: LinkedList<AllocationNode>,
}

impl IndexAllocator {
	
	pub fn new(start: usize)->Self{
		return Self{
			count: start,
			nodes: LinkedList::new(),
		};
	}
	/// Returns a new index on demand
	pub fn reserve_index(&mut self)->usize{
		match self.nodes.front_mut(){
			Some(node) => {
				if node.count > 1 {
					let current = node.start;
					node.start += 1;
					node.count -= 1;
					return current;
				} else {
					let current = node.start;
					self.nodes.pop_front();
					return current;
				}
			}
			None => {
				let current = self.count;
				self.count += 1;
				return current;
			}
		}
	}
	/// "Deallocates" a previously produced index. The index value is stored in a linked list of deallocated intervals and can be returned as another allocated index later on.
	pub fn free_index(&mut self, index: usize)->bool{
		if self.nodes.is_empty(){
			self.nodes.push_front(AllocationNode{start: index, count: 1});
			return true;
		}
		let len = self.nodes.len();
		for (idx, node) in self.nodes.iter_mut().enumerate(){
			if *node == index {
				if index == node.start + node.count {
					node.count += 1;
					return true;
				}
				return false;
			}
			if idx == 0 && *node > index {
				// insert a new node in front
				self.nodes.push_back(AllocationNode{start: index, count: 1});
				return true;
			}
			if idx == len-1 && index < self.count {
				// insert a new node at the back
				self.nodes.push_front(AllocationNode{start: index, count: 1});
				return true;
			}
		}
		return false;
	}
}


