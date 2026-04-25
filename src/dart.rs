// cmap::dart.rs

//! Definition of [`Dart`] data structure and basic manipulation routines, independent on other darts or the containing map.
//! Each dart stores information about its alpha connections required to build up a generalised map. The maximum number of dimensions is defined by the generic const parameter `N`.
//! Optionally, there might be attributes attached to *I*-cells of a map. As each cell generally includes more than one dart, all darts must point to the attribute data stored elsewhere.
//! There is an mechanism synchonizing those index pointers for darts belonging to the same cell whenever a modification in the map happens.
//! The maximum number of different *I* for which attributes can be stored is defined by the second generic const parameter `NA`.
//! Sometimes, full-blown connections between darts with all possible *I*-cells are not necessary, but darts still must be able to refer to each other. To do that, there is a second type of connections which do not follow the conventional alpha-connection rules.
//! They are called weak links and they usage is user-defined. For example, one might desire to define a linked-list like structure with isolated darts which are only connected to each other by means of weak links. We avoid the memory overhead related to initializing too many darts but are still able to store vertex attribute data.
//! The maximum number of weak links in a dart is defined by the third generic const parameter, `NL`.

use std::fmt;

use serde::{Serialize,Deserialize,ser::{Serializer},de::{Deserializer}};

use crate::index::{DartIndex,AttributeIndex, ATTRIBUTE_INDEX_NULL};
use crate::array_serialize::{Arr};

#[derive(Serialize,Deserialize)]
pub struct Dart<const N: usize, const NA: usize, const NL: usize>{
	init: bool,
	alphas: Arr<DartIndex,N>,
	attributes: Arr<AttributeIndex,NA>,
	links: Arr<DartIndex,NL>,
}

impl<const N: usize, const NA: usize, const NL: usize> Dart<N,NA,NL>{
	
	pub fn new_at(index: usize, nmax: &usize)->Self{
		return Self{
			init: true,
			alphas: std::array::from_fn(|idx| {if idx < *nmax {index.into()} else {0.into()}}).into(),
			attributes: std::array::from_fn(|_| AttributeIndex::null()).into(),
			links: std::array::from_fn(|_| DartIndex::from(index)).into(),
		};
	}
	
	pub fn get_alpha(&self, alpha: usize)->DartIndex{
		assert!(alpha < N);
		return self.alphas[alpha];
	}
	
	pub fn set_alpha(&mut self, other: &DartIndex, alpha: usize){
		assert!(alpha < N);
		self.alphas[alpha] = *other;
	}
	
	
	pub fn set_init(&mut self, init_marker: bool){
		self.init = init_marker;
	}
	
	pub fn is_init(&self)->bool{
		return self.init;
	}
	
	pub fn set_attribute_index(&mut self, index: AttributeIndex, idim: usize){
		assert!(idim < NA);
		self.attributes[idim] = index;
	}
	
	pub fn get_attribute_index(&self, idim: usize)->AttributeIndex{
		assert!(idim < NA);
		return self.attributes[idim];
	}
	
	pub fn is_attribute_nonfree(&self, idim: usize)->bool{
		return &self.attributes[idim] != &*ATTRIBUTE_INDEX_NULL;
	}
	
	pub fn set_link(&mut self, other: DartIndex, ilink: usize){
		assert!(ilink < NL);
		self.links[ilink] = other;
	}
	
	pub fn get_link(&self, ilink: usize)->DartIndex{
		return self.links[ilink];
	}
	
	pub fn reallocate<const N1: usize, const NA1: usize, const NL1: usize>(self)->Dart<N1,NA1,NL1>{
		let mut alphas = [0.into();N1];
		let mut attributes = [0.into();NA1];
		let mut links = [0.into();NL1];
		alphas[0..N].copy_from_slice(&*self.alphas);
		attributes[0..NA].copy_from_slice(&*self.attributes);
		links[0..NL].copy_from_slice(&*self.links);
		return Dart::<N1,NA1,NL1>{
			init: self.init,
			alphas: alphas.into(),
			attributes: attributes.into(),
			links: links.into(),
		};
		todo!();
	}
	
}

impl<const N: usize, const NA: usize, const NL: usize> fmt::Debug for Dart<N,NA,NL>{
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result{
		if N > 0 {
			formatter.write_str(&format!("A{:?} ", &*self.alphas))?;
		}
		if NA > 0 {
			formatter.write_str(&format!("E{:?} ", &*self.attributes))?;
		}
		if NL > 0 {
			formatter.write_str(&format!("L{:?} ", &*self.links))?;
		}
		return Ok(());
	}
}