// gmap::gmap.rs

use crate::index::{DartIndex};
use super::{NGMap};

impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	pub(super) fn get_link_raw(&self, dart: &DartIndex, ilink: usize)->DartIndex{
		return self.darts[**dart].get_link(ilink);
	}
	
	pub fn get_link(&self, dart: &DartIndex, ilink: usize)->Option<DartIndex>{
		if !self.darts[**dart].is_init() {return None;}
		let link = self.darts[**dart].get_link(ilink);
		if *link == 0 || &link == dart {return None;}
		return Some(link);
	}
	
	pub fn set_link(&mut self, dart: &DartIndex, link: DartIndex, ilink: usize){
		self.darts[**dart].set_link(link, ilink);
	}
}
