// gmap::gmap.rs

use crate::index::{DartIndex};
use super::{NGMap};


/// Removal operation consists in removing an *I*-cell from a map while merging two neighbouring *I*+1-cells into one. Removing is not always possible. For example, a vertex connected to three edges is not removable.
/// If the neighbouring *I*+1-cells contain attribute information, an `onmerge` functor is applied to obtain the attribute of a new *I*+1 cell.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	/// Returns `true` if the *I*-cell can be removed.
	pub fn is_removable(&self, dart: &DartIndex, idim: usize)->bool{
		assert!(idim < self.ndims);
		let n = self.ndims;
		if idim == n {return false;}
		if idim == n-1 {return true;}
		// TODO fix idim + 2
		for di in self.iter_darts_in_cell(*dart, idim){
			let d1 = self.get_alpha(&self.get_alpha(&di, idim+2), idim+1);
			let d2 = self.get_alpha(&self.get_alpha(&di, idim+1), idim+2);
			if d1 != d2 {return false;}
		}
		return true;
	}
	
	/// Removes the cell; returns `true` is the operation was successful
	pub fn remove_cell(&mut self, dart: &DartIndex, idim: usize)->bool{
		if !self.is_removable(dart, idim){return false;}
		let mark = self.reserve_mark();
		let darts : Vec<DartIndex> = self.iter_darts_in_cell(*dart, idim).map(|di| {self.mark(&di, &mark); di}).collect();
		let mut da1 : Option<DartIndex> = None;
		let mut da2 : Option<DartIndex> = None;
		for di in darts.iter(){
			let d1 = self.get_alpha(&di, idim);
			if !self.is_marked(&d1, &mark){
				let mut d2 = self.get_alpha(&self.get_alpha(&di, idim+1), idim);
				while self.is_marked(&d2, &mark){
					d2 = self.get_alpha(&self.get_alpha(&d2, idim+1), idim);
				}
				self.darts[*d1].set_alpha(&d2, idim);
				da1 = Some(d1);
				da2 = Some(d2);
			}
		}
		for di in darts.iter(){
			for k in 0..N {
				self.darts[**di].set_alpha(di, k);
			}
			assert!(self.remove_isolated_dart(*di));
		}
		// now we can merge the attributes
		self.merge_attributes(&da1.unwrap(), &da2.unwrap(), idim);
		return true;
	}
}
