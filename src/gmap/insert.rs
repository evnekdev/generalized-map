// gmap::gmap.rs

use std::collections::{HashMap};

//use crate::dart::{Dart};
use crate::orbits::{get_seq_sewing};
use crate::index::{DartIndex};
use super::{NGMap};

impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	pub fn is_insertable(&self, dart1: &DartIndex, dart2: &DartIndex, idim: usize, assoc: &HashMap<DartIndex, DartIndex>)->bool{
		assert!(idim < self.ndims);
		let number_icells = self.iter_adjacent(*dart2, idim).count();
		// TODO check if the conditions below are correctly applied
		//println!("number icells = {}", &number_icells);
		//if number_icells > 1 {return false;}
		//if !self.is_removable(dart2, idim){return false;}
		//println!("is_removable = {}", &self.is_removable(dart2, idim));
		// build inverse assoc map
		let mut associ : HashMap<DartIndex, DartIndex> = HashMap::new();
		assoc.iter().for_each(|(k,v)| {associ.insert(*v, *k);});
		let seq = get_seq_sewing(idim, N);
		for (d1, d2) in assoc.iter(){
			//println!("is_free {}", &self.is_free(d1, idim));
			if !self.is_free(d1, idim){return false;}
			for j in &seq {
				let d1_ = self.get_alpha(d1, *j);
				let d2_ = self.get_alpha(d2, *j);
				if assoc.get(&d1_).is_some_and(|d| d != &d2_){return false;}
			}
			for j in &seq {
				let d1_ = self.get_alpha(d1, *j);
				let d2_ = self.get_alpha(d2, *j);
				if associ.get(&d2_).is_some_and(|d| d != &d1_){return false;}
			}
			let mut d = self.get_alpha(&d1, idim+1);
			while !assoc.contains_key(&d){
				d = self.get_alpha(&self.get_alpha(&d, idim), idim+1);
			}
			if assoc.get(&d).is_some_and(|dd| dd != &self.get_alpha(&d2, idim)){return false;}
		}
		return true;
	}
	
	pub fn insert(&mut self, dart1: &DartIndex, dart2: &DartIndex, idim: usize, assoc: &HashMap<DartIndex, DartIndex>)->bool{
		if !self.is_insertable(dart1, dart2, idim, assoc){return false;}
		for (d1, d2) in assoc.iter(){
			self.darts[**d2].set_alpha(d1, idim);
			self.darts[**d1].set_alpha(d2, idim);
		}
		// TODO split attributes!!!!
		return true;
	}
}