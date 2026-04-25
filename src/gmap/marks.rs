// gmap::gmap.rs

use std::cell::{Ref, RefMut, RefCell};

use crate::index::{DartIndex, MarkIndex};
use super::{NGMap};

/// Interface to allocate and deallocate new marks. Many algorithms require to distinguish a previously processed dart to avoid processing it twice.
/// To do that, processed darts are *marked*. Two and more independent marks might be required at the same time, so there must be a mechanism to distinguish between them.
/// Each mark has its own id [`MarkIndex`] which should be allocated and released when not used anymore.
/// In other implementations, marks are bit fields in a dart structure itself which imposes restrictions on the amount of marks which can be processed simultaneously.
/// The current implementation builds an internal boolean vector of the same size for each mark which is destroyed when a mark is freed, eliminating the need to iterate over all darts and clean up mark flags.
impl<const N: usize, const NA: usize, const NL: usize> NGMap<N,NA,NL> {
	
	/// Reserves a storage for boolean marks for darts.
	pub fn reserve_mark(&self)->MarkIndex{
		let vec: Vec<bool> = vec![false;self.darts.len()];
		let mark = MarkIndex::from(self.marks.borrow().len());
		self.marks.borrow_mut().insert(*mark, RefCell::new(vec));
		return mark;
	}
	
	/// Releases memory occupied by marks of `mark` index
	pub fn free_mark(&self, mark: MarkIndex)->bool{
		return self.marks.borrow_mut().remove(&*mark).is_some();
	}
	
	/// Releases memory occupied by all marks
	pub fn free_marks(&self){
		self.marks.borrow_mut().retain(|_,_| false);
	}
	
	/// Returns `true` if a `dart` is marked using mark index `mark`.
	pub fn is_marked(&self, dart: &DartIndex, mark: &MarkIndex)->bool{
		let val = self.marks.borrow().get(&**mark).is_some_and(|marks| marks.borrow()[**dart]);
		return val;
	}
	
	/// Marks `dart` using mark index `mark`
	pub fn mark(&self, dart: &DartIndex, mark: &MarkIndex)->bool{
		return self.marks.borrow().get(&**mark).is_some_and(|marks| {marks.borrow_mut()[**dart] = true; true});
	}
	
	/// Unmarks `dart` using mark index `mark`
	pub fn unmark(&self, dart: &DartIndex, mark: &MarkIndex)->bool{
		return self.marks.borrow().get(&**mark).is_some_and(|marks| {marks.borrow_mut()[**dart] = false; true});
	}
	
	/// marks all darts in an *I*-cell
	pub fn mark_cell(&self, dart: &DartIndex, mark: &MarkIndex, idim: usize)->bool{
		self.iter_darts_in_cell(*dart, idim).for_each(|d| {self.mark(&d, mark);});
		return true;
	}
	/// Unmarks all darts in an *I*-cell
	pub fn unmark_cell(&self, dart: &DartIndex, mark: &MarkIndex, idim: usize)->bool{
		self.iter_darts_in_cell(*dart, idim).for_each(|d| {self.unmark(&d, mark);});
		return true;
	}
}
