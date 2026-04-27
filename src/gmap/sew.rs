// gmap::gmap.rs


use std::collections::{HashMap};

use crate::orbits::{get_seq_sewing};
use crate::index::{DartIndex};
use super::{NGMap};


/// This section deals with sewing and unsewing darts. Sewing is a dart handling operation joining darts together. Not every combination of dart connections satisfies the constaints of being a valid generalized map.
/// Therefore, sewing two darts together might be accompanied by sewing the corresponding neighbouring darts and so on. The sewing operation is not always possible for an arbitrary pair of darts.
/// Darts must be different and alpha(*I*)-free; also, according to the definition of the sewing operation, the *orbits* [alpha(*0*), ..., alpha(*I*-2),alpha(*I*+2),...,alpha(*N*)] for both darts must be isomorphic to each other.
/// Unlike sewing, unsewing is always possible for a pair of distinct darts.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	/// Returns `true` if `dart` and `dart1` span isomorphic orbits, called by [`Self::sew`]
	pub fn is_sewable(&self, dart1: &DartIndex, dart2: &DartIndex, alpha: usize)->bool{
		if dart1 == dart2 || !self.is_free(dart1, alpha){
			return false;
		}
		let seq = get_seq_sewing(alpha, self.ndims);
		let mut it1 = self.iter_orbit(*dart1, &seq);
		let mut it2 = self.iter_orbit(*dart2, &seq);
		let mut assoc : HashMap<DartIndex, DartIndex> = HashMap::new();
		loop {
			match (it1.next(), it2.next()){
				(Some(dart1),Some(dart2)) => {
					assoc.insert(dart1, dart2);
					for j in &seq{
						let dart1_j = self.get_alpha(&dart1, *j);
						let dart2_j = self.get_alpha(&dart2, *j);
						match assoc.get(&dart1_j){
							Some(&ref res) => {
								if &dart2_j != res {return false;}
							}
							None => {}
						}
					}
				}
				(Some(_),None)|(None,Some(_)) => {
					return false;
				}
				(None,None) => {break;}
			}
		}
		return true;
	}
	
	/// Glues two i-cells together, effectively making them one i-cell.
	/// This operation is only possible if `dart1` and `dart2` are different,
	/// 
	/// both are alpha(i)-free and the orbits spanned by them are isomorphic to each other (it is not possible to sew a triangle and a square faces together).
	pub fn sew(&mut self, dart1: &DartIndex, dart2: &DartIndex, alpha: usize)->bool{
		if !self.is_sewable(dart1, dart2, alpha){return false;}
		let same_cell_1 : Vec<bool> = (0..N).into_iter().map(|idim| self.darts_in_same_cell(dart1, dart2, idim)).collect();
		let seq = get_seq_sewing(alpha, self.ndims);
		let it1 = self.iter_orbit(*dart1, &seq);
		let it2 = self.iter_orbit(*dart2, &seq);
		let pairs : Vec<(DartIndex, DartIndex)> = it1.zip(it2).collect();
		for (dart1, dart2) in pairs {
			self.darts[*dart1].set_alpha(&dart2, alpha);
			self.darts[*dart2].set_alpha(&dart1, alpha);
		}
		let same_cell_2 : Vec<bool> = (0..N).into_iter().map(|idim| self.darts_in_same_cell(dart1, dart2, idim)).collect();
		for idim in 0..N {
			if self.has_attribute_container(idim) && same_cell_1[idim] != same_cell_2[idim]{
				let index = self.merge_attributes(dart1, dart2, idim);
			}
		}
		return true;
	}
	
	/// Unglues an i-cell into its separate components.
	pub fn unsew(&mut self, dart1: &DartIndex, dart2: &DartIndex, alpha: usize){
		let seq = get_seq_sewing(alpha, N);
		let it1 = self.iter_orbit(*dart1, &seq);
		let it2 = self.iter_orbit(*dart2, &seq);
		let pairs : Vec<(DartIndex, DartIndex)> = it1.zip(it2).collect();
		for (dart1, dart2) in pairs {
			self.darts[*dart1].set_alpha(&dart1, alpha);
			self.darts[*dart2].set_alpha(&dart2, alpha);
		}
		//self.update_embeddings_on_split(dart1, dart2);
		todo!();
	}
}
