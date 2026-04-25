// gmap::gmap.rs

use crate::dart::{Dart};
use crate::index::{DartIndex};

use super::{NGMap};

/// Deals with the creation and removal of darts and access interface to individual darts.
/// Each dart can be thought of like a half of an 1Dedge. Depending on the dimensionality of the geometrical object, each dart belongs to one and only one *I*-cell for each *I* = 0, 1, ... *N*.
/// A dart belongs to a unique vertex (*I* = 0), edge (*I* = 1), face (*I* = 2), volume (*I* = 3), etc. Each dart has *alpha* connections which link it to other darts. These connections are set up
/// in such a way that upon jumping through alpha(*I*), we move to a dart which belongs to the same *J*-cells except for *J* = *I*. If a dart in not connected to anything for a certain value of *I* (it is called *I*-free),
///, the convention is that it is alpha(*I*)-connected to itself. Although unusual, this formalism can be easily expanded into higher dimensions and all algorithms remain the same.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	/// Returns `true` if `dart` is not alpha(i)-connected to another dart.
	pub fn is_free(&self, dart: &DartIndex, aindex: usize)->bool{
		return &self.darts[**dart].get_alpha(aindex) == dart;
	}
	
	/// Creates a new isolated dart with no alpha(i) connections to other darts and returns its index [`crate::index::DartIndex`].
	pub fn create_dart(&mut self)->DartIndex{
		let index = self.dart_index_allocator.borrow_mut().reserve_index();
		let dart = Dart::new_at(index, &self.ndims);
		//println!("dart = {:?}", &dart);
		if index < self.darts.len(){
			self.darts[index] = dart;
		} else {
			self.darts.push(dart);
			for (_, mark) in &*self.marks.borrow(){
					mark.borrow_mut().push(false);
			}
		}
		for idim in 0..self.ndims {
			match self.reserve_attribute_index(idim){
				Some(idx) => {
					self.set_attribute_index_to_cell(&index.into(), idx.into(), idim);
				}
				None => {
					// just skip
				}
			}
		}
		return index.into();
	}
	
	/// Removes a dart in case it is isolated (not alpha(i)-connected to other darts).
	/// If the operation was successful, returns the dart index wrapped in [`Option`]
	/// TODO - maybe change the return type to [`bool`]?
	pub fn remove_isolated_dart(&mut self, dart: DartIndex)->bool{
		if *dart == 0 {return false;}
		for k in 0..N {
			if !self.is_free(&dart, k){return false;}
		}
		for idim in 0..N {
			self.remove_attribute_from_dart(&dart, idim);
		}
		if *dart == self.darts.len()-1 {
			self.darts.pop();
		} else {
			self.darts[*dart].set_init(false);
			//self.free_dart_indices.push_back(*dart);
			self.dart_index_allocator.borrow_mut().free_index(*dart);
		}
		return true;
	}
	
	/// Returns `true` if `dart1` and `dart2` belong to the same i-cell.
	pub fn darts_in_same_cell(&self, dart1: &DartIndex, dart2: &DartIndex, idim: usize)->bool{
		if dart1 == dart2 {return true;}
		for d in self.iter_darts_in_cell(*dart1, idim){
			if &d == dart2{return true;}
		}
		return false;
	}
	
	/// Retrieves dart index connected to `dart` via alpha(i).
	///
	/// There is a convention saying that if a dart is alpha(i)-free, its alpha(i)-"connected" to itself, so there is always a valid output to this method.
	pub fn get_alpha(&self, dart: &DartIndex, alpha: usize)->DartIndex{
		return self.darts[**dart].get_alpha(alpha);
	}
	
	/// Returns index of a dart alpha(i)-connected to `dart` wrapped in [`Option`] only in case `dart` is not alpha(i)-free, `None` otherwise.
	pub fn get_alpha_if_exists(&self, dart: &DartIndex, alpha: usize)->Option<DartIndex>{
		let dart_alpha = self.darts[**dart].get_alpha(alpha);
		if &dart_alpha == dart {return None;}
		return Some(dart_alpha);
	}
	
	/// Returns a dart index belonging to a next edge
	/// 
	/// TODO finish
	pub fn next(&self, dart: &DartIndex)->DartIndex{
		//return self.get_alpha(dart, 1);
		todo!();
	}
	
	/// Returns a dart index belonging to a previous edge
	/// 
	/// TODO finish
	pub fn prev(&self, dart: &DartIndex)->DartIndex{
		//return self.get_alpha(&self.opposite(dart), 1);
		todo!();
	}
	
	/// TODO
	pub fn opposite(&self, dart: &DartIndex)->DartIndex{
		//return self.get_alpha(dart, 0);
		todo!();
	}
	
	pub fn darts_connected(&self, dart1: &DartIndex, dart2: &DartIndex)->bool{
		if dart1 == dart2 {return true;}
		for dart in self.iter_darts_in_component(*dart1){
			if dart2 == &dart {
				return true;
			}
		}
		return false;
	}
}
