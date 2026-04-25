// gmap::gmap.rs

use std::cmp;
use std::any::{TypeId};
use std::collections::{HashSet};

use crate::index::{DartIndex, ATTRIBUTE_INDEX_NULL};
use super::{NGMap};

/// Occupation statitics of the map
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	/// Counts *I*-cells in the whole map.
	pub fn number_cells(&self, idim: usize)->usize{
		return self.iter_dart_per_cell(idim).count();
	}
	
	pub(super) fn number_dimensions_non_free(&self)->(usize,usize,usize){
		let mut alpha_nonfree : [bool;N] = [false;N];
		let mut attribute_nonfree : [bool;NA] = [false;NA];
		let mut pointer_nonfree : [bool;NL] = [false;NL];
		
		for dart in self.iter_all(){
			for k in 0..self.ndims {
				if self.darts[*dart].get_alpha(k) != dart{
					alpha_nonfree[k] = true;
				}
			}
			for k in 0..NA {
				if self.darts[*dart].get_attribute_index(k) != *ATTRIBUTE_INDEX_NULL{
					attribute_nonfree[k] = true;
				}
			}
			for k in 0..NL {
				if self.darts[*dart].get_link(k) != dart {
					pointer_nonfree[k] = true;
				}
			}
		}
		
		return (alpha_nonfree.into_iter().filter(|val| *val).count(),
				attribute_nonfree.into_iter().filter(|val| *val).count(),
				pointer_nonfree.into_iter().filter(|val| *val).count(),
		);
	}
	
	/// Returns the total number of alpha slots involved at least once throughout the map
	pub fn number_alpha_nonfree(&self)->usize{
		return self.ndims;
	}
	/*
	pub fn number_alpha_nonfree(&self)->usize{
		let mut alpha_nonfree : [bool;N] = [false;N];
		for dart in self.iter_all(){
			for k in 0..N {
				if self.darts[*dart].get_alpha(k) != dart{
					alpha_nonfree[k] = true;
				}
			}
		}
		return alpha_nonfree.into_iter().filter(|val| *val).count();
	}
	*/
	/// Returns the total number of alpha slots involved at least once in the current connected component
	pub fn number_alpha_nonfree_component(&self, dart: &DartIndex)->usize{
		let mut alpha_nonfree : [bool;N] = [false;N];
		for dart in self.iter_darts_in_component(*dart){
			for k in 0..N {
				if self.darts[*dart].get_alpha(k) != dart {
					alpha_nonfree[k] = true;
				}
			}
		}
		return alpha_nonfree.into_iter().filter(|val| *val).count();
	}

	pub(super) fn number_distinct_dimensions<const N1: usize, const NA1: usize, const NL1: usize>(&self, other: &NGMap<N1,NA1,NL1>)->(usize,usize){
		let mut typeset : HashSet<(usize, TypeId)> = HashSet::new();
		for k in 0..N {
			let _ = self.get_attribute_container(k).is_some_and(|cnt| typeset.insert((k, cnt.get_key_type_id())));
		}
		for k in 0..N1 {
			let _ = other.get_attribute_container(k).is_some_and(|cnt| typeset.insert((k, cnt.get_key_type_id())));
		}
		// TODO figure out how to work out NL links
		return (typeset.len(), cmp::max(NL, NL1));
		todo!();
	}
	
	/// Counts the connected components in the map
	pub fn number_connected_components(&self)->usize{
		return self.iter_dart_per_component().count();
	}
}
