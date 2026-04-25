// gmap::lib.rs

#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_variables)]

#![doc(html_no_source)]

//#![feature(trait_upcasting)]

//#![feature(const_generics)]
//#![feature(const_evaluatable_checked)]
//#![feature(generic_const_exprs)]

pub mod dart;
pub mod gmap;
pub mod index;
pub mod index_allocator;
pub mod iterators;
pub mod orbits;
pub mod attribute;
pub mod inspect;
pub mod output;
pub mod array_serialize;

pub use crate::dart::{Dart};
pub use crate::index::{DartIndex, MarkIndex, AttributeIndex};
pub use crate::gmap::{NGMap, IteratorAllDarts};
pub use crate::iterators::{IteratorOrbit, IteratorDartPerCell, IteratorDartPerCellIncident, IteratorDartPerCellAdjacent};

