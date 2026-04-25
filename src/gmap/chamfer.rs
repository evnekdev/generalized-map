// gmap::gmap.rs

use std::collections::{HashMap};

use crate::index::{DartIndex};
use super::{NGMap};

/// Chamfering consists in replacing an *I*-cell with a *J*-cell. For instance, vertices can be replaced by faces to make the object look smoother.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	/// Performs the chamfer operation.
	pub fn chamfer(&mut self, d: &DartIndex, idim: usize, n: usize){
		assert!(idim < self.ndims);
		let mut phi : Vec<HashMap<DartIndex,DartIndex>> = vec![HashMap::new();n+1];
		let darts : Vec<DartIndex> = self.iter_darts_in_cell(*d, idim).collect();
		for dart in darts.iter(){
			// line 3
			phi[idim].insert(*dart, *dart);
			for j in (idim+1)..n+1{
				// line 5
				phi[j].insert(*dart, self.create_dart());
			}
		}
		for dart in darts.iter(){
			for j in (idim+1)..n+1 {
				for k in 0..idim {
					// line 9
					let di = phi[j].get(dart).unwrap();
					let dia = self.get_alpha(dart, k);
					let di1 = phi[j].get(&dia).unwrap();
					self.darts[**di].set_alpha(di1, k);
				}
				for k in idim..j {
					// line 11
					let di = phi[j].get(dart).unwrap();
					let dia= self.get_alpha(dart, k+1);
					let di1 = phi[j].get(&dia).unwrap();
					self.darts[**di].set_alpha(di1, k);
				}
				// line 12
				let di = phi[j].get(dart).unwrap();
				let di1 = phi[j-1].get(dart).unwrap();
				self.darts[**di].set_alpha(di1, j);
				if j < n {
					// line 14
					let di = phi[j].get(dart).unwrap();
					let di1 = phi[j+1].get(dart).unwrap();
					self.darts[**di].set_alpha(di1, j+1);
				}
				for k in (j+2)..n+1 {
					// line 16
					let di = phi[j].get(dart).unwrap();
					let dia= self.get_alpha(dart, k);
					let di1= phi[j].get(&dia).unwrap();
					self.darts[**di].set_alpha(di1, k);
				}
			}
		}
		
		for dart in darts.iter(){
			let di = phi[idim+1].get(dart).unwrap();
			self.darts[**dart].set_alpha(di, idim+1);
		}
	}
	
}
