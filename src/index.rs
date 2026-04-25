// cmap::index.rs

//! Type encapsulation for indices of darts, marks, and attributes. Internally, each index type represents an `usize`, but the type encapsulation is required to make use of the Rust type system and avoid confusion between indices with different meanings.

use std::ops::{Deref};

use std::fmt;

use lazy_static::{lazy_static};

use serde::{Serialize,Deserialize,ser::{Serializer},de::{Deserializer}};

use crate::dart::{Dart};

lazy_static! {
	pub static ref ATTRIBUTE_INDEX_NULL : AttributeIndex = AttributeIndex::null();
}

/**************************************************************************************************************************/

#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct DartIndex(usize);

impl From<usize> for DartIndex{
	fn from(other: usize)->Self {
		return DartIndex(other);
	}
}

impl Deref for DartIndex{
	type Target = usize;
	
	fn deref(&self)->&Self::Target{
		return &self.0;
	}
}

impl fmt::Debug for DartIndex{
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result{
		return formatter.write_str(&format!("d{}", &self.0));
	}
}

impl fmt::Display for DartIndex{
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result{
		return formatter.write_str(&format!("d{}", &self.0));
	}
}

/**************************************************************************************************************************/

pub struct MarkIndex(usize);

impl From<usize> for MarkIndex{
	fn from(other: usize)->Self {
		/*
		if other >= NMARKS_MAX {
			panic!("Cannot create a mark from an index exceeding the maximum number of marks allowed!");
		}
		*/
		return MarkIndex(other);
	}
}

impl Deref for MarkIndex {
	type Target = usize;
	
	fn deref(&self)->&Self::Target{
		return &self.0;
	}
}

impl fmt::Debug for MarkIndex{
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result{
		return formatter.write_str(&format!("m{}", &self.0));
	}
}

impl fmt::Display for MarkIndex{
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result{
		return formatter.write_str(&format!("m{}", &self.0));
	}
}

/**************************************************************************************************************************/
#[derive(Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct AttributeIndex(usize);

impl AttributeIndex{
	
	pub fn null()->Self{
		return Self(usize::MAX);
	}
}

impl From<usize> for AttributeIndex{
	fn from(source: usize)->Self{
		return Self(source);
	}
}

impl Deref for AttributeIndex {
	type Target = usize;
	
	fn deref(&self)->&Self::Target{
		return &self.0;
	}
}

impl fmt::Debug for AttributeIndex{
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result{
		let str_ : String = if self.0 == usize::MAX {
			format!("e??")
		} else {
			format!("e{}", &self.0)
		};
		return formatter.write_str(&str_);
	}
}

impl fmt::Display for AttributeIndex{
	fn fmt(&self, formatter: &mut fmt::Formatter)->fmt::Result{
		let str_ : String = if self.0 == usize::MAX {
			format!("e??")
		} else {
			format!("e{}", &self.0)
		};
		return formatter.write_str(&str_);
	}
}

