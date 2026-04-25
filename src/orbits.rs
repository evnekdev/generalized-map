// cmap::orbits.rs

//! Utility functions to generate orbit sequences for iterators and map modification operations.

pub fn get_seq(i: usize, n: usize)->Vec<usize>{
	assert!(i < n);
	let mut iseq: Vec<usize> = Vec::new();
		for k in 0..i {
			iseq.push(k);
		}
		for k in i..n-1{
			iseq.push(k+1);
		}
	return iseq;
}


pub fn get_seq_sewing(i: usize, n: usize)->Vec<usize>{
	assert!(i < n);
	let mut iseq: Vec<usize> = Vec::new();
	if i >= 2 {
		for k in 0..i-1{
			iseq.push(k);
		}
	}
	for k in i+2..n{
		iseq.push(k);
	}
	return iseq;
}