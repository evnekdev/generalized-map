// gmap::gmap.rs

use crate::index::{DartIndex};
use crate::orbits::{get_seq_sewing};
use super::{NGMap};

/// *Closure* operation fills in open boundaries, or *closes* them. The operation is performed by identifying alpha(*I*)-free darts and connecting them to new darts.
/// If needed, the new darts are connected to each other to follow the boundary surface.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	/// Performs *I*-closure on a map.
	pub fn close_boundary(&mut self, idim: usize){
		assert!(idim < self.ndims);
		let seq = get_seq_sewing(idim, self.ndims);
		let darts : Vec<DartIndex> = self.iter_all().filter(|d| self.is_free(&d, idim)).collect();
		for dart in darts.iter(){
			let d1 = self.create_dart();
			self.darts[**dart].set_alpha(&d1,idim);
			self.darts[*d1].set_alpha(&dart, idim);
			for j in seq.iter(){
				let d2 = self.get_alpha(&dart, *j);
				if !self.is_free(dart,*j) && !self.is_free(&d2,idim){
					self.darts[*d1].set_alpha(&d2, idim);
					self.darts[*d2].set_alpha(&d1, idim);
				}
			}
			if idim > 0 {
				let mut d2 = self.get_alpha(&dart, idim-1);
				while !self.is_free(&d2, idim) && !self.is_free(&self.get_alpha(&d2,idim), idim-1){
					d2 = self.get_alpha(&self.get_alpha(&d2, idim), idim-1);
				}
				if !self.is_free(&d2, idim){
					let da1 = self.get_alpha(&d2, idim);
					self.darts[*d1].set_alpha(&da1, idim-1);
					let da2 = self.get_alpha(&d2, idim);
					self.darts[*da2].set_alpha(&d1, idim-1);
				}
			}
		}
		todo!();
	}
}
