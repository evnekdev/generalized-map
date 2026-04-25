// gmap::inspect.rs

//! Utility code for single *dart* navigation. The main structure, [`DartInspector`], encapsulates a [`DartIndex`] and a [`NGMap`] reference and allows to chain together simple "walk" methods with no arguments.

use crate::index::{DartIndex};
use crate::gmap::{NGMap};

pub struct DartInspector<'a, const N: usize, const NA: usize, const NL: usize>{
	index: DartIndex,
	map: &'a NGMap<N, NA, NL>,
}


impl<'a, const N: usize, const NA: usize, const NL: usize> DartInspector<'a, N, NA, NL>{
	pub fn new(index: DartIndex, map: &'a NGMap<N,NA,NL>)->Self{
		return Self{
			index: index,
			map: map,
		};
	}
	
	pub fn release(self)->DartIndex{
		return self.index;
	}
	
	pub fn peek(&self)->&DartIndex{
		return &self.index;
	}
	
	pub fn alpha(self, idim: usize)->Option<Self>{
		let index = self.map.get_alpha(&self.index, idim);
		if index != self.index {
			return Some(Self::new(index, self.map));
		}
		return None;
	}
	
	pub fn next(self)->Option<Self>{
		return self.alpha(0)?.alpha(1);
	}
	
	pub fn prev(self)->Option<Self>{
		return self.alpha(1)?.alpha(0);
	}
	
	pub fn rot_ccw(self)->Option<Self>{
		return self.alpha(2)?.alpha(0);
	}
	
	pub fn rot_cw(self)->Option<Self>{
		return self.alpha(0)?.alpha(2);
	}
	
	pub fn forward(self)->Option<Self>{
		return self.rot_ccw()?.rot_ccw()?.alpha(1);
	}
	
	pub fn back(self)->Option<Self>{
		return self.alpha(1)?.rot_ccw()?.rot_ccw();
	}
	
	pub fn reflect(self)->Option<Self>{
		return self.rot_ccw()?.rot_ccw();
	}
	
}
