// cmap::attribute.rs

//! This module provides a generalized map with a generic container for attributes. In a generalized map, each *I*-cell can have an optional attribute attached to it which can be almost anything (coordinates, properties, colors, etc.).
//! If for a certain *I*, *I*-cells may contain attributes, these attributes have its own user-defined type. Cells with different *I*'s might have unrelated attributes of different types, which however, still have to be stored in a uniform manner.
//! The Rust strong typing system would force us to define this attribute types as additional type parameters for the map. To avoid that, this submodule provides a generic data storage type ([`AttributeContainer`]) which encapsulates specifics of a stored data type using dynamic dyspatch and [`std::any::Any`] trait functionality.

use std::fmt;
use std::fmt::{Debug};
use std::collections::{HashMap, HashSet};
use std::any::{Any, TypeId};
use std::ops::{Deref, DerefMut};

use downcast_rs::{Downcast, impl_downcast};

use serde::{Serialize,Deserialize,ser::{Serializer},de::{Deserializer}};

use crate::index::{AttributeIndex, ATTRIBUTE_INDEX_NULL};


/****************************************************************************************/

/// This trait only exposes the type-independent methods
pub trait ContainerGeneric : Downcast + Debug {
	
	fn len_(&self)->usize;
	
	fn contains_key_(&self, key: &AttributeIndex)->bool;
	
	fn merge_data(&mut self, indices_old: (AttributeIndex, AttributeIndex), index_new: AttributeIndex)->bool;
	
	fn remove_data(&mut self, key: AttributeIndex)->bool;
	
	fn split_data(&mut self, index_old: AttributeIndex, indices_new: (AttributeIndex, AttributeIndex))->bool;
	
	fn copy_data_from(&mut self, index: AttributeIndex, other: &AttributeContainer, index_other: &AttributeIndex)->bool;
}

impl_downcast!(ContainerGeneric);

/****************************************************************************************/
/// This structure implements all type-dependent methods
pub struct InnerContainer<T>
where T: Debug + Clone,
{
	pub onmerge : Box<dyn Fn(T,T)->T>,
	pub onsplit : Box<dyn Fn(T)->(T,T)>,
	pub map: HashMap<AttributeIndex,T>,
}

impl<T> InnerContainer<T>
where T: Debug + Clone,
{
	
	pub fn new(onmerge: Box<dyn Fn(T,T)->T>, onsplit: Box<dyn Fn(T)->(T,T)>)->Self{
		return Self{
			onmerge: onmerge,
			onsplit: onsplit,
			map: HashMap::new(),
		};
	}
}

impl<T> Debug for InnerContainer<T>
where T : Debug + Clone,
{
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result{
		return self.map.fmt(formatter);
	}
}

impl<T: Debug + Clone + 'static> ContainerGeneric for InnerContainer<T>{
	fn len_(&self)->usize{
		return self.map.len();
	}
	
	fn contains_key_(&self, key: &AttributeIndex)->bool{
		return self.map.contains_key(key);
	}
	
	fn merge_data(&mut self, indices_old: (AttributeIndex,AttributeIndex), index_new: AttributeIndex)->bool{
		if index_new == *ATTRIBUTE_INDEX_NULL {return true;}
		let (idx0, idx1) = indices_old;
		match (self.map.remove(&idx0), self.map.remove(&idx1)){
			(Some(data0), Some(data1)) => {
				let data = (&self.onmerge)(data0,data1);
				self.map.insert(index_new, data);
				return true;
			}
			(Some(data0),None) | (None,Some(data0)) => {
				self.map.insert(index_new, data0);
				return true;
			}
			(None,None) => {return false;}
		}
	}
	
	fn remove_data(&mut self, key: AttributeIndex)->bool{
		if key == *ATTRIBUTE_INDEX_NULL {return false;}
		return self.map.remove(&key).is_some();
	}
	
	fn split_data(&mut self, index_old: AttributeIndex, indices_new: (AttributeIndex,AttributeIndex))->bool{
		let (idx0, idx1) = indices_new;
		if idx0 == *ATTRIBUTE_INDEX_NULL || idx1 == *ATTRIBUTE_INDEX_NULL {return false;}
		match self.map.remove(&index_old){
			Some(data) => {
				let (data0, data1) = (&self.onsplit)(data);
				self.map.insert(idx0, data0);
				self.map.insert(idx1, data1);
				return true;
			}
			None => {return false;}
		}
	}
	
	fn copy_data_from(&mut self, index: AttributeIndex, other: &AttributeContainer, index_other: &AttributeIndex)->bool{
		match other.get_data::<T>(index_other){
			Some(data_) => {
				let data = (*data_).clone();
				self.map.insert(index, data);
			}
			None => {return false;}
		}
		todo!();
	}
}

/****************************************************************************************/
/// This is the envelope type erasing the stored type
pub struct AttributeContainer{
	key_type_id: TypeId,
	assoc: Box<dyn ContainerGeneric>,
}

impl AttributeContainer {
	pub fn new<T: Debug + Clone + 'static>(onmerge: Box<dyn Fn(T,T)->T>, onsplit: Box<dyn Fn(T)->(T,T)>)->Self{
		return Self{
			key_type_id: TypeId::of::<T>(),
			assoc: Box::new(InnerContainer::<T>::new(onmerge, onsplit)),
		};
	}
	
	/// A set method with a generic type parameter identifying the data type.
	pub fn set_data<T: Debug + Clone + 'static>(&mut self, index: AttributeIndex, data: T)->bool{
		//println!("setting data {:?} to {:?}", &data, &index);
		if index == *ATTRIBUTE_INDEX_NULL {return false;}
		match self.assoc.downcast_mut::<InnerContainer<T>>(){
			Some(ref mut map) => {
				map.map.entry(index).or_insert(data);
				return true;
			}
			None => {
				return false;
				}
		}
	}
	/// A get method with a generic type parameter. If the type does not match the internally stored type or there is no data stored for the given index, returns [`None`].
	pub fn get_data<T: Debug + Clone + 'static>(&self, index: &AttributeIndex)->Option<&T>{
		if index == &*ATTRIBUTE_INDEX_NULL {return None;}
		return self.assoc.downcast_ref::<InnerContainer<T>>()?.map.get(index);
	}
	/// Returns `true` if the container is not empty
	pub fn has_data(&self, index: &AttributeIndex)->bool{
		if index == &*ATTRIBUTE_INDEX_NULL {return false;}
		return self.assoc.contains_key_(index);
	}
	/// Total number of attributes stored in the container
	pub fn len(&self)->usize{
		return self.assoc.len_();
	}
	/// Removes data, if any, for an index. Returns `true` if the removal was successful.
	pub fn remove_data(&mut self, index: AttributeIndex)->bool{
		return self.assoc.remove_data(index);
	}
	/// Applies an `onmerge` functor to two instances of `T` data type, which must be defined in case if during an operation on the map two *I*-cells merge into one.
	pub fn merge_data(&mut self, indices_old: (AttributeIndex,AttributeIndex), index_new: AttributeIndex)->bool{
		return self.assoc.merge_data(indices_old, index_new);
	}
	
	/// Applies an `onsplit` functor to an instance of `T` data type, which must be defined in case if during an operation on the map an *I*-cell gets split into two.
	pub fn split_data(&mut self, index_old: AttributeIndex, indices_new: (AttributeIndex,AttributeIndex))->bool{
		return self.assoc.split_data(index_old, indices_new);
	}
	
	/// Clones data from another container into the current one and replaces the attribute index. The data in the original container remains untouched. To perform this operation, the generic data type `T` must implement [`Clone`] trait.
	pub fn copy_data_from(&mut self, index: AttributeIndex, other: &AttributeContainer, index_other: &AttributeIndex)->bool{
		return self.assoc.copy_data_from(index, other, index_other);
	}
	
	/// Returns [`TypeId`] (see [`std::any`]) of the internally stored data type `T`.
	pub fn get_key_type_id(&self)->TypeId{
		return self.key_type_id.clone();
	}
	
	/// Checks if the input generic type parameter matches the internally stored one.
	pub fn same_key_type<T: Debug + Clone + 'static>(&self, key: T)->bool{
		return self.get_key_type_id() == key.type_id();
	}
}

impl Debug for AttributeContainer {
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result {
		return (&*self.assoc).fmt(formatter);
	}
}

/****************************************************************************************/
/****************************************************************************************/

pub fn onmerge_default<T: Debug + Clone + 'static>(data1: T, data2: T)->T{
	return data1;
}

pub fn onsplit_default<T: Debug + Clone + 'static>(data: T)->(T,T){
	return (data.clone(), data);
}

/****************************************************************************************/

pub fn onmerge_default_boxed<T: Debug + Clone + 'static>()->Box<dyn Fn(T,T)->T>{
	return Box::new(onmerge_default);
}

pub fn onsplit_default_boxed<T: Debug + Clone + 'static>()->Box<dyn Fn(T)->(T,T)>{
	return Box::new(onsplit_default);
}