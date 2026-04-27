// gmap::gmap.rs

//! Generalized map is a powerful datastructure which allows to represent space subdivisions of n-dimensional objects. It is a generalization of a half-edge data structure.
//! A space subdivision is represented as a set of *darts* connected in a special way. The n-dimensional space subdivision is viewed as a collection of simpler n-dimensional *cells*, optionally glued together along through pairs of (n-1)-dimensional cells.
//! Each n-dimensional cell has a *boundary* tiled with (n-1)-dimensional cells, each (n-1)-dimensional cell has its own boundary of (n-2) dimensional cells, and so on. 0-cells are the same as geometrical vertices, 1-cells represent edges, 2-cells - faces, etc.
//! Each *I*-cell is comprised of *darts*, each *dart* belongs to only one cell for each *I*. The connections between *darts* allow to switch cells for one *I* at a time.

use std::cmp;
use std::cell::{Ref, RefMut, RefCell};
use std::collections::{VecDeque, HashMap, HashSet};

use serde::{Serialize,Deserialize,ser::{Serializer},de::{Deserializer}};
use serde_json;

use crate::dart::{Dart};
use crate::index::{DartIndex};
use crate::iterators::{IteratorOrbit, IteratorDartsInCell, IteratorDartPerCell, IteratorDartPerCellIncident, IteratorDartPerCellAdjacent, IteratorDartPerComponent, IteratorDartsInComponent};
use crate::attribute::{AttributeContainer};
use crate::index_allocator::{IndexAllocator};
use crate::array_serialize::{Arr};

mod marks;
mod attributes;
mod darts;
mod sew;
mod close_boundary;
mod remove;
mod contract;
mod expand;
mod insert;
mod chamfer;
mod extrude;
mod triangulate;
mod links;
mod stats;
mod shapes;
mod shapes_embedded;

/// Generalized map structure representing topological relations between geometrical objects in arbitrary N dimensions
/// using a universal approach of *darts* (or half-edges), connection between which (alpha-links)
/// describe vertices, edges, faces, volumes and so on (referred to as i-cells).
/// Optionally, the structure can store additional information attached to i-cells (embeddings, or attributes).
/// 
/// Type parameters:
/// 
/// *N* is the maximum number of dimensions
/// 
/// *NA* is the maximum number of available attributes (act as "slots", any combinations of NA i of i-cells which embeddings are possible)
/// 
/// *NL* is the maximum number of additional weak links between darts which facilitate searching on the map and are not part of the stardard alpha-link interface.
#[derive(Serialize)]
pub struct NGMap<const N: usize, const NA: usize = 0, const NL: usize = 0>{
	// N is the maximum dimension,
	// NA is the maximum number of attributes
	// NL is the maximum number of "extra" links to other darts (hierarchy structures, etc.)
	ndims: usize, // less or equal to N
	darts: Vec<Dart<N,NA,NL>>,
	//free_dart_indices: VecDeque<usize>,
	//#[serde(skip_serializing)]
	dart_index_allocator: RefCell<IndexAllocator>,
	//#[serde(skip_serializing)]
	marks: RefCell<HashMap<usize,RefCell<Vec<bool>>>>,
	//#[serde(skip_serializing)]
	attribute_container_indices: HashMap<usize, usize>,
	#[serde(skip_serializing)]
	attribute_containers: [Option<RefCell<AttributeContainer>>;NA],
	//#[serde(skip_serializing)]
	attribute_index_allocators: Arr<Option<RefCell<IndexAllocator>>,NA>,
}

/// Generalized map creation, copying, clearing and iterating.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	fn _default_attribute_containers()->[Option<RefCell<AttributeContainer>>;NA]{
		return std::array::from_fn(|_| None);
	}
	
	/// Initializes an empty generalized map
	pub fn new(ndims: usize)->Self{
		return NGMap{
					ndims: ndims,
					darts: vec![Dart::new_at(0, &1)],
					dart_index_allocator: IndexAllocator::new(1).into(),
					//free_dart_indices: VecDeque::new(),
					marks: HashMap::new().into(),
					attribute_container_indices: HashMap::new(),
					attribute_containers: std::array::from_fn(|_| None),
					attribute_index_allocators: std::array::from_fn(|_| None).into(),
					};
	}
	
	/// Clears out the contents of the map, removing all darts, freeing marks and attributes
	pub fn free_all(&mut self){
		self.free_marks();
		self.free_attribute_containers();
		self.darts = Vec::new();
		self.dart_index_allocator = IndexAllocator::new(0).into();
		//self.free_dart_indices.retain(|_| false);
	}
	
	/// Returns an iterator over an arbitrary orbit of [alpha(i1), alpha(i2), ... alpha(ik)] starting with `dart`.
	/// 
	/// The most generic iterator which is heavily used by over iterators.
	pub fn iter_orbit<'a>(&'a self, dart: DartIndex, iseq: &'a [usize])->IteratorOrbit<'a, N,NA,NL>{
		return IteratorOrbit::new(&self, dart, iseq);
	}
	
	/// Returns an iterator over all darts in an i-cell.
	pub fn iter_darts_in_cell(&self, dart: DartIndex, i: usize)->IteratorDartsInCell<'_, N,NA,NL>{
		return IteratorDartsInCell::new(&self, dart, i);
	}
	
	/// Returns an iterator which iterates over exactly one dart per i-cell in the generalized map.
	pub fn iter_dart_per_cell(&self, i: usize)->IteratorDartPerCell<'_, N,NA,NL>{
		return IteratorDartPerCell::new(&self, i);
	}
	
	/// Returns an incident iterator, which iterates over exactly one dart per i-cell incident to a j-cell
	/// (all vertices incident to a face, all faces incident to a volume, all faces incident to a vertex, etc.)
	/// 
	/// Please not that i and j must be different, otherwise refer to [`Self::iter_adjacent`]
	pub fn iter_incident(&self, dart: DartIndex, i: usize, j: usize)->IteratorDartPerCellIncident<'_, N,NA,NL>{
		return IteratorDartPerCellIncident::new(&self, dart, i, j);
	}
	
	/// Returns an adjacency iterator which iterates over exactly one dart per an i-cell neighbouring the i-cell to which `dart` belongs.
	pub fn iter_adjacent(&self, dart: DartIndex, i: usize)->IteratorDartPerCellAdjacent<'_, N,NA,NL>{
		return IteratorDartPerCellAdjacent::<N,NA,NL>::new(&self, dart, i);
	}
	
	/// Returns an iterator over all darts in the map
	pub fn iter_all(&self)->IteratorAllDarts<'_, N,NA,NL>{
		return IteratorAllDarts::new(&self);
	}
	
	pub fn iter_dart_per_component(&self)->IteratorDartPerComponent<'_, N,NA,NL>{
		return IteratorDartPerComponent::new(&self);
	}
	
	pub fn iter_darts_in_component(&self, dart: DartIndex)->IteratorDartsInComponent<'_, N,NA,NL>{
		return IteratorDartsInComponent::new(&self, dart);
	}
	
	// TODO finalise the function
	pub fn copy_from<const N1: usize, const NA1: usize, const NL1: usize>(&mut self, other: &NGMap<N1, NA1, NL1>)->bool{
		if self.has_attribute_collisions(other){return false;}
		let (nattributes, npointers) = self.number_distinct_dimensions(other);
		if nattributes > NA {return false;}
		if npointers > NL {return false;} // TODO elaborate
		self.free_marks(); // free all marks because after adding darts from another map they won't make sense anyway
		// TODO make sure we have the same set of attribute containers!
		let mark = self.reserve_mark();
		let mut dart_assoc : HashMap<DartIndex,DartIndex> = HashMap::new();
		// get number of cells in the initial map
		let ncells : Vec<usize> = (0..NA).into_iter().map(|idx| self.number_cells(idx)).collect();
		for darto in other.iter_all(){
			let dart = self.create_dart();
			self.mark(&dart, &mark);
			dart_assoc.insert(darto, dart);
		}
		for (darto, dart) in dart_assoc.iter(){
			for k in 0..cmp::min(N, N1){
				let alpha = other.get_alpha(darto, k);
				self.darts[**dart].set_alpha(&alpha, k);
			}
			// TODO for now, we assume that all links have the same role in both collections
			for k in 0..cmp::min(NL, NL1){
				let alpha = other.get_link_raw(darto, k);
				self.darts[**dart].set_link(alpha, k);
			}
		}
		// prepare attribute indices for new darts
		for idim in 0..NA {
			match self.attribute_container_indices.get(&idim) {
				Some(idx) => {
					let mut count = self.iter_all().filter(|dart| !self.is_marked(&dart, &mark)).map(|dart| self.get_attribute_index(&dart, idim)).fold(0, |acc, index| cmp::max(acc, *index));
					// with that, proceed numbering cells for the new darts
					let darts_new : Vec<DartIndex> = self.iter_all().filter(|dart| self.is_marked(&dart, &mark)).collect();
					let mark1 = self.reserve_mark();
					for dart in darts_new.into_iter(){
						if !self.is_marked(&dart, &mark1){
							let darts_ : Vec<DartIndex> = self.iter_darts_in_cell(dart, idim).collect();
							for di in darts_.into_iter(){
								self.darts[*di].set_attribute_index(count.into(), *idx);
								self.mark(&di, &mark1);
							}
							count += 1;
						}
					}
				}
				None => {continue;}
			}
		}
		// copy attributes
		for idim in 0..NA {
			match (self.get_attribute_container_mut(idim), other.get_attribute_container(idim)) {
				(Some(mut cnt1), Some (cnt2)) => {
					for (dart, darto) in dart_assoc.iter() {
						let index = self.get_attribute_index(dart, idim);
						let indexo= other.get_attribute_index(darto, idim);
						if !cnt1.has_data(&index) && cnt2.has_data(&indexo){
							cnt1.copy_data_from(index, &cnt2, &indexo);
						}
					}
				}
				(Some(_), None) => {
					// nothing to do
				}
				(None, Some(_)) => {
					todo!(); // shouldn't happen
				}
				(None, None) => {
					continue;
				}
			}
		}
		self.free_marks(); // free all marks because after adding darts from another map they won't make sense anyway
		return true;
		//todo!();
	}
	
	pub fn copy_to<const N1: usize, const NA1: usize, const NL1: usize>(&self, other: &mut NGMap<N1, NA1, NL1>)->bool{
		return other.copy_from(&self);
	}
	
	/*
	// instead of increaseDimNGMap and decreaseDimNGMap, we rather join them into one operation
	pub fn change_dim<const N1: usize, const NA1: usize, const NL1: usize>(self)->NGMap<N1,NA1,NL1>{
		let mut new = NGMap::<N1,NA1,NL1>::new();
		self.copy_to(&mut new);
		return new;
	}
	*/
	pub fn change_dim(&mut self, ndims: usize){
		if ndims > N {
			panic!("Cannot increase dimensions to {}, consider reallocating storage", &ndims);
		}
		if ndims < self.ndims {
			for di in 1..self.darts.len(){
				for k in ndims..self.ndims {
					self.darts[di].set_alpha(&0.into(), k);
				}
			}
		}
		if ndims > self.ndims {
			for di in 1..self.darts.len(){
				for k in self.ndims..ndims{
					self.darts[di].set_alpha(&di.into(), k);
				}
			}
		}
		self.ndims = ndims;
	}
	
	pub fn reallocate<const N1: usize, const NA1: usize, const NL1: usize>(mut self)->NGMap<N1,NA1,NL1>{
		if N1 < N {
			todo!();
		}
		if NA1 < NA {
			todo!();
		}
		if NL1 < NL {
			todo!();
		}
		let mut darts : Vec<Dart<N1,NA1,NL1>> = Vec::with_capacity(self.darts.len());
		for dart in self.darts.into_iter(){
			darts.push(dart.reallocate());
		}
		let mut attribute_containers :   [Option<RefCell<AttributeContainer>>;NA1] = std::array::from_fn(|_| None);
		let mut attribute_index_allocators : [Option<RefCell<IndexAllocator>>;NA1] = std::array::from_fn(|_| None);
		for k in 0..NA {
			attribute_containers[k]       = self.attribute_containers[k].take();
			attribute_index_allocators[k] = self.attribute_index_allocators[k].take();
		}
		return NGMap{
			ndims: self.ndims,
			darts: darts,
			dart_index_allocator: self.dart_index_allocator,
			marks: self.marks,
			attribute_container_indices: self.attribute_container_indices,
			attribute_containers: attribute_containers.into(),
			attribute_index_allocators: attribute_index_allocators.into(),
		}
	}
	
	/// TODO clarify the function signature
	pub fn restrict(&mut self){
		todo!();
	}
	
	pub fn print(&self){
		println!("{:?}\n{:?}", &self.darts, &self.attribute_containers);
	}
	
	pub fn to_json(&self)->String{
		return serde_json::to_string(self).unwrap();
	}
}


/***********************************************************************************************************************/
/***********************************************************************************************************************/


/// Iterates over all valid darts in the entire map.
pub struct IteratorAllDarts<'a, const N: usize, const NA: usize, const NL: usize>{
	map: &'a NGMap<N,NA,NL>,
	cur: usize,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> IteratorAllDarts<'a,N,NA,NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>)->Self{
		return Self{map: map, cur: 1};
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for IteratorAllDarts<'a,N,NA,NL>{
	type Item = DartIndex;
	
	fn next(&mut self)->Option<Self::Item>{
		let cur = self.cur;
		if cur < self.map.darts.len(){
			self.cur += 1;
			if !self.map.darts[cur].is_init(){
				return self.next();
			}
			return Some(cur.into());
		}
		return None;
	}
}

fn _bool_true()->bool{return true;}