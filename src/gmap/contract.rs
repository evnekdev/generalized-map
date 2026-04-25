// gmap::lib.rs

use crate::index::{DartIndex};
use super::{NGMap};

/// *Contraction* operation is similar to *removal*. An *I*-cell is removed from the map while the neighbouring (*I*-1)-cells are merged into one.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL>{
	
	/// Returns `true` if the *I*-cell can be contracted.
	pub fn is_contractible(&self, dart: &DartIndex, idim: usize)->bool{
		assert!(idim < self.ndims);
		if idim == 0 {return false;}
		if idim == 1 {return true;}
		for dart in self.iter_darts_in_cell(*dart, idim){
			let da1 = self.get_alpha(&self.get_alpha(&dart, idim-2),idim-1);
			let da2 = self.get_alpha(&self.get_alpha(&dart, idim-1),idim-2);
			if da1 != da2 {return false;}
		}
		return true;
	}
	
	/// Contracts the *I*-cell, if possible
	pub fn contract(&mut self, dart: &DartIndex, idim: usize){
		assert!(idim < self.ndims);
		let mark = self.reserve_mark();
		let darts : Vec<DartIndex> = self.iter_darts_in_cell(*dart, idim).map(|d| {self.mark(&d, &mark); d}).collect();
		for dart in darts.iter(){
			let d1 = self.get_alpha(dart, idim);
			if !self.is_marked(&d1, &mark){
				let mut d2 = self.get_alpha(&self.get_alpha(dart, idim-1), idim);
				while self.is_marked(&d2, &mark){
					d2 = self.get_alpha(&self.get_alpha(&d2, idim-1), idim);
				}
				self.darts[*d1].set_alpha(&d2, idim);
			}
		}
		for dart in darts.iter(){
			for k in 0..N{
				self.darts[**dart].set_alpha(dart,k);
			}
			self.remove_isolated_dart(*dart);
		}
		self.free_mark(mark);
	}
	
}