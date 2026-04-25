// cmap::dart.rs

//! General-purpose iterators for generalized maps for iterating over *orbits*, *cells*, etc.

use std::cell::RefCell;

use crate::dart::{Dart};
use crate::orbits::{get_seq};
use crate::index::{DartIndex, MarkIndex};
use crate::gmap::{NGMap, IteratorAllDarts};
use crate::inspect::{DartInspector};

/**************************************************************************************************************************/

/// The most generic iterator, iterates over all darts belonging to an *orbit* of the initial dart. An *orbit* is defined as a subset of *alpha* connections of a dart limited to indices from [alpha(*I*0), alpha(*I*1), ..., alpha(*I*k)].
pub struct IteratorOrbit<'a, const N: usize, const NA: usize, const NL: usize>{
	mark: Option<MarkIndex>,
	//dindex: usize,
	map: &'a NGMap<N,NA,NL>,
	iseq: &'a [usize],
	stack: Vec<DartIndex>,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> IteratorOrbit<'a,N,NA,NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>, dart: DartIndex, iseq: &'a [usize])->Self{
		assert!(iseq.len() <= N);
		let mark = map.reserve_mark();
		map.mark(&dart, &mark);
		map.mark(&0.into(), &mark);
		return Self{mark: Some(mark), map: map, iseq: iseq, stack: vec![dart]};
	}
}

/// A custom [`Drop`] implementation is required to free marks internally created by the iterator.
impl<'a, const N: usize, const NA: usize, const NL: usize> Drop for IteratorOrbit<'a, N, NA, NL>{
	fn drop(&mut self){
		// free the mark before the iterator is dropped
		let mark = self.mark.take();
		self.map.free_mark(mark.unwrap());
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for IteratorOrbit<'a, N, NA, NL>{
	type Item = DartIndex;
	
	fn next(&mut self)->Option<Self::Item>{
		let cur = self.stack.pop()?;
		for k in 0..self.iseq.len() {
			let next = self.map.get_alpha(&cur, self.iseq[k]);
			if next != cur && !self.map.is_marked(&next, self.mark.as_ref()?){
				self.map.mark(&next, self.mark.as_ref()?);
				self.stack.push(next);
			}
		}
		return Some(cur);
	}
}

/**************************************************************************************************************************/

/// Iterates over all darts which belong to an  *I*-cell: a vertex, an edge, a face, a volume, etc. This is done by performing an *orbit* iteration over all *alpha*'s except for *alpha*(*I*).
pub struct IteratorDartsInCell<'a, const N: usize, const NA: usize, const NL: usize>{
	map: &'a NGMap<N,NA,NL>,
	inner: Box<dyn Iterator<Item=DartIndex>>,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> IteratorDartsInCell<'a, N, NA, NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>, dart: DartIndex, i: usize)->Self{
		let iseq = get_seq(i, N);
		let vec : Vec<DartIndex> = IteratorOrbit::<N,NA,NL>::new(map, dart, &iseq).collect();
		return Self{
			map: map,
			inner: Box::new(vec.into_iter()),
		};
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for IteratorDartsInCell<'a,N,NA,NL>{
	type Item = DartIndex;
	fn next(&mut self)->Option<Self::Item>{
		return self.inner.next();
	}
}

/**************************************************************************************************************************/

/// Iterates over all *I*-cells (vertices, edges, faces, volumes, etc), in the map by yielding one dart per cell.
pub struct IteratorDartPerCell<'a, const N: usize, const NA: usize, const NL: usize>{
	mark: Option<MarkIndex>,
	map: &'a NGMap<N,NA,NL>,
	inner: Box<dyn Iterator<Item=DartIndex>>,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> IteratorDartPerCell<'a, N,NA,NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>, i: usize)->Self{
		let mark = map.reserve_mark();
		map.mark(&0.into(), &mark);
		let iseq = get_seq(i, N);
		let mut vec : Vec<DartIndex> = Vec::new();
		for dart in map.iter_all(){
			if !map.is_marked(&dart, &mark){
				for d in map.iter_orbit(dart, &iseq){
					//println!("d = {}, is_marked = {}", &d, map.is_marked(&d, &mark));
					if !map.is_marked(&d, &mark){
						map.mark(&d, &mark);
					}
				}
				vec.push(dart);
			}
		}
		return Self{mark: Some(mark), map: map, inner: Box::new(vec.into_iter())};
	}
}

/// A custom [`Drop`] implementation is required to free marks internally created by the iterator.
impl<'a, const N: usize, const NA: usize, const NL: usize> Drop for IteratorDartPerCell<'a, N, NA, NL>{
	fn drop(&mut self){
		let mark = self.mark.take();
		self.map.free_mark(mark.unwrap());
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for IteratorDartPerCell<'a, N, NA, NL>{
	type Item = DartIndex;
	
	fn next(&mut self)->Option<Self::Item>{
		return self.inner.next();
	}
}

/**************************************************************************************************************************/

/// Iterates over *J*-cells incident to an *I*-cell by yielding one dart per *J*-cell.
/// For example, to iterate over all vertices incident to a volume, use *I* = 3 and *J* = 0; to iterate over all volumes which share a vertex, use *I* = 0 and *J* = 3.
/// Indices *I* and *J* must be different.
pub struct IteratorDartPerCellIncident<'a, const N: usize, const NA: usize, const NL: usize>{
	mark : Option<MarkIndex>,
	map: &'a NGMap<N,NA,NL>,
	inner: Box<dyn Iterator<Item=DartIndex>>,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> IteratorDartPerCellIncident<'a, N, NA, NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>, dart: DartIndex, i: usize, j: usize)->Self{
		assert!(i != j);
		let mark = map.reserve_mark();
		map.mark(&0.into(), &mark);
		let iseq = get_seq(i, N);
		let jseq = get_seq(j, N);
		let mut vec: Vec<DartIndex> = Vec::new();
		for d in map.iter_orbit(dart, &jseq){
			if !map.is_marked(&d, &mark){
				for di in map.iter_orbit(d, &iseq){
					map.mark(&di, &mark);
				}
				vec.push(d);
			}
		}
		return Self{mark: Some(mark), map: map, inner: Box::new(vec.into_iter())};
	}
}

/// A custom [`Drop`] implementation is required to free marks internally created by the iterator.
impl<'a, const N: usize, const NA: usize, const NL: usize> Drop for IteratorDartPerCellIncident<'a, N, NA, NL>{
	fn drop(&mut self){
		let mark = self.mark.take();
		self.map.free_mark(mark.unwrap());
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for IteratorDartPerCellIncident<'a, N, NA, NL>{
	type Item = DartIndex;
	
	fn next(&mut self)->Option<Self::Item>{
		return self.inner.next();
	}
}

/**************************************************************************************************************************/

/// Iterates over all *I*-cells adjacent to the initial *I*-cell by yielding one dart per cell.
pub struct IteratorDartPerCellAdjacent<'a, const N: usize, const NA: usize, const NL: usize>{
	mark : Option<MarkIndex>,
	map: &'a NGMap<N,NA,NL>,
	inner: Box<dyn Iterator<Item=DartIndex>>,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> IteratorDartPerCellAdjacent<'a,N,NA,NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>, dart: DartIndex, i: usize)->Self{
		let mark = map.reserve_mark();
		map.mark(&0.into(), &mark);
		let iseq = get_seq(i,N);
		let mut vec: Vec<DartIndex> = Vec::new();
		for d in map.iter_orbit(dart, &iseq){
			let d_i = map.get_alpha(&d, i);
			if !map.is_marked(&d_i, &mark){
				for dd in map.iter_orbit(d_i,&iseq){
					map.mark(&dd, &mark);
				}
				vec.push(d_i);
			}
		}
		return Self{mark: Some(mark), map: map, inner: Box::new(vec.into_iter())};
	}
}

/// A custom [`Drop`] implementation is required to free marks internally created by the iterator.
impl<'a, const N: usize, const NA: usize, const NL: usize> Drop for IteratorDartPerCellAdjacent<'a, N, NA, NL>{
	fn drop(&mut self){
		let mark = self.mark.take();
		self.map.free_mark(mark.unwrap());
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for IteratorDartPerCellAdjacent<'a, N, NA, NL>{
	type Item = DartIndex;
	
	fn next(&mut self)->Option<Self::Item>{
		return self.inner.next();
	}
}

/**************************************************************************************************************************/

/// Iterates over each connected component of the generalized map by yielding exactly one dart (unspecified which one) per connected component.
pub struct IteratorDartPerComponent<'a, const N: usize, const NA: usize, const NL: usize>{
	mark: Option<MarkIndex>,
	map: &'a NGMap<N,NA,NL>,
	inner: Box<dyn Iterator<Item=DartIndex>>,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> IteratorDartPerComponent<'a, N, NA, NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>)->Self{
		let mark = map.reserve_mark();
		map.mark(&0.into(), &mark);
		let seq: Vec<usize> = (0..N).into_iter().collect();
		let mut vec: Vec<DartIndex> = Vec::new();
		for dart in map.iter_all(){
			if !map.is_marked(&dart, &mark){
				for di in map.iter_orbit(dart, &seq){
					if !map.is_marked(&di, &mark){
						map.mark(&di, &mark);
					}
				}
				vec.push(dart);
			}
		}
		return Self{
			mark: Some(mark),
			map: map,
			inner: Box::new(vec.into_iter()),
		};
	}
}

/// A custom [`Drop`] implementation is required to free marks internally created by the iterator.
impl<'a, const N: usize, const NA: usize, const NL: usize> Drop for IteratorDartPerComponent<'a,N,NA,NL>{
	fn drop(&mut self){
		let mark = self.mark.take();
		self.map.free_mark(mark.unwrap());
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for IteratorDartPerComponent<'a, N, NA, NL>{
	type Item = DartIndex;
	fn next(&mut self)->Option<Self::Item>{
		return self.inner.next();
	}
}

/**************************************************************************************************************************/

/// Iterates over all darts in the connected component which contains the initial dart. In case if all darts in the map are interconnected, this iterator is equivalent to [`crate::IteratorAllDarts`].
pub struct IteratorDartsInComponent<'a, const N: usize, const NA: usize, const NL: usize>{
	map: &'a NGMap<N,NA,NL>, // keeps the map from changing while the iterator exists
	inner: Box<dyn Iterator<Item=DartIndex>>,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> IteratorDartsInComponent<'a, N,NA,NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>, dart: DartIndex)->Self{
		let iseq : Vec<usize> = (0..N).into_iter().collect();
		let vec: Vec<DartIndex> = map.iter_orbit(dart, &iseq).collect();
		return Self {
			map: map,
			inner: Box::new(vec.into_iter()),
		};
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for IteratorDartsInComponent<'a, N,NA,NL>{
	type Item=DartIndex;
	fn next(&mut self)->Option<Self::Item>{
		return self.inner.next();
	}
}

/**************************************************************************************************************************/

/// Iterates over darts exiting the same vertex (??? what about 3D and nD cases?)
pub struct Circulator<'a, const N: usize, const NA: usize, const NL: usize>{
	inspector: Option<DartInspector<'a, N, NA, NL>>,
	dart: DartIndex,
	started: bool,
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Circulator<'a, N, NA, NL>{
	pub fn new(map: &'a NGMap<N,NA,NL>, dart: DartIndex)->Self{
		return Self{inspector: Some(DartInspector::new(dart, map)), dart: dart, started: false};
	}
}

impl<'a, const N: usize, const NA: usize, const NL: usize> Iterator for Circulator<'a, N,NA,NL>{
	type Item = DartIndex;
	
	fn next(&mut self)->Option<Self::Item>{
		if self.started && self.inspector.as_ref()?.peek() == &self.dart {return None;}
		if !self.started {self.started = true;}
		let current = *self.inspector.as_ref()?.peek();
		self.inspector = self.inspector.take()?.rot_ccw();
		return Some(current);
	}
}
