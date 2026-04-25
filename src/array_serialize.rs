// gmap::array_serialize.rs

use std::fmt;
use std::marker::{PhantomData};
use std::fmt::{Debug};
use std::ops::{Deref, DerefMut};

use serde::{Serialize,Deserialize, ser::{Serializer, SerializeSeq}, de::{Deserializer, Error, Visitor, SeqAccess}};

/*****************************************************************************/
/*****************************************************************************/

struct ArrVisitor<'de, T : Serialize, const N: usize>{_p: PhantomData<&'de T>,}

impl<'de, T, const N: usize> ArrVisitor<'de, T, N>
where T: Serialize + Deserialize<'de> + Debug,
{
	pub fn new()->Self{
		return ArrVisitor{_p: PhantomData};
	}
}

impl<'de, T, const N: usize> Visitor<'de> for ArrVisitor<'de, T, N>
where T: Serialize + for<'a> Deserialize<'a> + Debug,
{
	type Value = Arr<T,N>;
	
	fn expecting(&self, formatter: &mut fmt::Formatter)->fmt::Result {
		formatter.write_str(&format!("An array with {} elements", N))
	}
	
	fn visit_seq<V>(self, mut seq: V)->Result<Arr<T,N>,V::Error>
	where V: SeqAccess<'de>,
	{
		let mut vec: Vec<T> = Vec::new();
		for k in 0..N {
			match seq.next_element()? {
				Some(t) => {
					vec.push(t);
				}
				None => {panic!("Not enough data!");}
			}
		}
		let res = vec.try_into().expect("Cannot convert to statically sized array");
		return Ok(Arr(res));
	}
}

/// Serializable wrapper around a generic array type
pub struct Arr<T: Serialize, const N: usize>(pub [T;N]);


impl<'de, T, const N: usize>Serialize for Arr<T,N>
	where T: Serialize + Deserialize<'de> + Debug,
	{
	fn serialize<S>(&self, serializer: S)->Result<S::Ok, S::Error>
	where S: Serializer {
		match serializer.serialize_seq(Some(N)){
			Ok(mut state) => {
				//println!("OK");
				for k in 0..N {
					//println!("serializing {:?}", &self.0[k]);
					state.serialize_element(&self.0[k])?;
				}
				return state.end();
			}
			Err(e) => {
				println!("ERR");
				return Err(e);
			}
		}
	}
}

impl<'de, T, const N: usize>Deserialize<'de> for Arr<T,N>
where T: Serialize + for<'a> Deserialize<'a> + Debug + 'de,
{
	fn deserialize<D>(deserializer: D)->Result<Self,D::Error>
	where D: Deserializer<'de>,
	{
		
		let arr = deserializer.deserialize_seq(ArrVisitor::new())?;
		return Ok(arr);
	}
}

impl<T,const N: usize> Deref for Arr<T,N>
	where T: Serialize + Debug,
	{
	type Target = [T;N];
	
	fn deref(&self)->&Self::Target{
		return &self.0;
	}
}

impl<T,const N: usize> DerefMut for Arr<T,N>
	where T: Serialize + Debug,
	{
	//type Target = [T;N];
	
	fn deref_mut(&mut self)->&mut Self::Target {
		return &mut self.0;
	}
}


impl<'de, T,const N: usize> From<[T;N]> for Arr<T,N>
where T: Serialize + for<'a> Deserialize<'a> + Debug + 'de,
{
	fn from(source: [T;N])->Self{
		return Self(source);
	}
}
/*****************************************************************************/
/*****************************************************************************/