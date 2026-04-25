// gmap::gmap.rs

use std::fmt::{Debug};
use std::collections::{HashMap};

use crate::index::{DartIndex};
use crate::attribute::{onmerge_default, onsplit_default};
use super::{NGMap};

impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	/// Extrudes the component represented by `dart1` along 1-D path represented by `dart2`
	pub fn extrude(&mut self, dart1: &DartIndex, dart2: &DartIndex, n1: usize)->Option<NGMap<N,NA,NL>>{
		return Some(self.extrude_(dart1, dart2, n1)?.0);
	}
	
	pub(super) fn extrude_(&mut self, dart1: &DartIndex, dart2: &DartIndex, n1: usize)->Option<(NGMap<N,NA,NL>, Vec<HashMap<(DartIndex,DartIndex),DartIndex>>)>
	{
		// TODO check that dart2 belongs to a 1D path
		//let n1 = self.number_alpha_nonfree_component(dart1)-1;
		//let n2 = self.number_alpha_nonfree_component(dart2)-1;
		// TODO fix it!
		//let n1 = 1;
		let n2 = 1;
		//println!("n1 = {}, n2 = {}", &n1, &n2);
		if self.darts_connected(dart1, dart2){return None;}
		//if n2 != 1 || n1 + 1 >= N {return None;}
		let n = n1;
		let mut phi : Vec<HashMap<(DartIndex,DartIndex), DartIndex>> = (0..n+1).into_iter().map(|_| HashMap::new()).collect();
		let darts1 : Vec<DartIndex> = self.iter_darts_in_component(*dart1).collect();
		let darts2 : Vec<DartIndex> = self.iter_darts_in_component(*dart2).collect();
		let mut rec = NGMap::<N,NA,NL>::new(self.ndims+1);
		for d1 in darts1.iter(){
			for d2 in darts2.iter(){
				for k in 0..n+1 {
					// lines 5-6
					//let nd = self.create_dart();
					let nd = rec.create_dart();
					phi[k].insert((*d1,*d2), nd);
				}
			}
		}
		for d1 in darts1.iter(){
			for d2 in darts2.iter(){
				// line 9
				let nd0 = phi[0].get(&(*d1,*d2)).unwrap();
				let d2a0 = self.get_alpha(d2, 0);
				//self.darts[**nd0].set_alpha(phi[0].get(&(d1,d2a0)).unwrap(), 0);
				rec.darts[**nd0].set_alpha(phi[0].get(&(*d1,d2a0)).unwrap(), 0);
				// line 10
				let nd1 = phi[1].get(&(*d1,*d2)).unwrap();
				//self.darts[**nd0].set_alpha(nd1, 1);
				rec.darts[**nd0].set_alpha(nd1, 1);
				
				for i in 2..n+2{
					// line 12
					let d1aim1 = self.get_alpha(d1, i-1);
					//self.darts[**nd0].set_alpha(phi[0].get(&(d1aim1, d2)).unwrap(), i);
					rec.darts[**nd0].set_alpha(phi[0].get(&(d1aim1,*d2)).unwrap(), i);
				}
				let ndn = phi[n].get(&(*d1,*d2)).unwrap();
				for i in 0..n {
					// line 14
					let d1ai = self.get_alpha(d1,i);
					//self.darts[**ndn].set_alpha(phi[n].get(&(d1ai,d2)).unwrap(), i);
					rec.darts[**ndn].set_alpha(phi[n].get(&(d1ai,*d2)).unwrap(), i);
				}
				// line 15
				let ndnm1 = phi[n-1].get(&(*d1, *d2)).unwrap();
				//self.darts[**ndn].set_alpha(ndnm1,n);
				rec.darts[**ndn].set_alpha(ndnm1,n);
				// line 16
				let d2a1 = self.get_alpha(d2,1);
				//self.darts[**ndn].set_alpha(phi[n].get(&(d1,d2a1)).unwrap(),n+1);
				rec.darts[**ndn].set_alpha(phi[n].get(&(*d1,d2a1)).unwrap(),n+1);
				
				for j in 1..n {
					let ndj = phi[j].get(&(*d1, *d2)).unwrap();
					for i in 0..j{
						// line 19
						let d1ai = self.get_alpha(d1, i);
						//self.darts[**ndj].set_alpha(phi[j].get(&(d1ai, d2)).unwrap(), i);
						rec.darts[**ndj].set_alpha(phi[j].get(&(d1ai, *d2)).unwrap(), i);
					}
					// line 20
					//self.darts[**ndj].set_alpha(phi[j-1].get(&(d1,d2)).unwrap(), j);
					rec.darts[**ndj].set_alpha(phi[j-1].get(&(*d1, *d2)).unwrap(), j);
					//line 21
					//self.darts[**ndj].set_alpha(phi[j+1].get(&(d1, d2)).unwrap(), j+1);
					rec.darts[**ndj].set_alpha(phi[j+1].get(&(*d1, *d2)).unwrap(), j+1);
					for i in (j+2)..n+2 {
						// line 23
						let ndaim1 = self.get_alpha(d1, i-1);
						//self.darts[**ndj].set_alpha(phi[j].get(&(ndaim1, d2)).unwrap(),i);
						rec.darts[**ndj].set_alpha(phi[j].get(&(ndaim1, *d2)).unwrap(),i);
					}
				}
			}
		}
		return Some((rec,phi));
	}
	
	pub fn extrude_with_vertex_attributes<T1,T2>(&mut self, dart1: &DartIndex, dart2: &DartIndex, n1: usize)->Option<NGMap<N,NA,NL>>
	where T1: Debug + Clone + 'static, T2: Debug + Clone + 'static,
	{
		let (mut map, phi) = self.extrude_(dart1, dart2, n1)?;
		map.reserve_attribute_container::<(T1,T2)>(0, Box::new(onmerge_default), Box::new(onsplit_default));
		// TODO derive onmerge and onsplit based on the existing onmerge and onsplit in the old map
		for ((d1,d2), d) in phi[0].iter(){
			//println!("d1 = {:?}, d2 = {:?}, d = {:?}", &d1, &d2, &d);
			//println!("attr = {:?}", &self.get_attribute_for_dart::<T1>(&d1, 0));
			match (self.get_attribute_for_dart::<T1>(&d1, 0),self.get_attribute_for_dart::<T2>(&d2, 0)){
				(Some(data1_),Some(data2_)) => {
					let data1 = data1_.clone();
					let data2 = data2_.clone();
					//println!("data1 = {:?}, data2 = {:?}", &data1, &data2);
					map.set_attribute_to_dart::<(T1,T2)>(&d, (data1,data2), 0);
				}
				_ => {
					// skip
				}
			}
		}
		return Some(map);
	}
}
