#![no_std]

//! Allows to extract a sub-array out of an array
//!
//! # Example
//!
//! Getting a sub array:
//!
//! ```
//! use sub_array::SubArray;
//!
//! let arr: [u8; 7] = [1, 2, 3, 4, 5, 6, 7];
//! let sub: &[u8; 3] = arr.sub_array_ref(1); // start at offset 1
//! assert_eq!(sub, &[2, 3, 4]);
//! ```
//!
//! Getting a mutable sub array:
//!
//! ```
//! use sub_array::SubArray;
//!
//! let mut arr = ["baz".to_string(), "qux".to_string(), "foo".to_string()];
//! let sub = arr.sub_array_mut::<1>(2); // get 1 element at offset 2
//! sub[0].push_str("bar");
//! assert_eq!(
//!     arr,
//!     ["baz".to_string(), "qux".to_string(), "foobar".to_string()]
//! );
//! ```


/// Array that can be slice into a smaller sub-array
///
/// Also see the [crate] level reference.
pub trait SubArray {
	/// The value type of this array
	type Item;

	/// Get a reference to a sub-array of length `N` starting at `offset`.
	///
	/// # Panics
	/// Panics if `offset + N` exceeds the length of this array.
	fn sub_array_ref<const N: usize>(&self, offset: usize) -> &[Self::Item; N];

	/// Get a mutable reference to a sub-array of length `N` starting at
	/// `offset`.
	///
	/// # Panics
	/// Panics if `offset + N` exceeds the length of this array.
	fn sub_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [Self::Item; N];
}

impl<T, const M: usize> SubArray for [T; M] {
	type Item = T;

	fn sub_array_ref<const N: usize>(&self, offset: usize) -> &[Self::Item; N] {
		self[offset..(offset + N)].try_into().unwrap()
	}

	fn sub_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [Self::Item; N] {
		(&mut self[offset..(offset + N)]).try_into().unwrap()
	}
}



#[cfg(test)]
mod tests {
	extern crate alloc;

	use alloc::string::String;
	use alloc::string::ToString;

	use super::*;


	#[test]
	fn empty_ref() {
		let arr = [0_u8; 0];
		assert_eq!(arr.sub_array_ref::<0>(0), &[]);
	}

	#[test]
	fn empty_mut() {
		let mut arr = [0_u8; 0];
		assert_eq!(arr.sub_array_mut::<0>(0), &mut []);
	}

	#[test]
	fn full_ref() {
		let arr = [1, 2, 3_i8];
		assert_eq!(arr.sub_array_ref::<3>(0), &[1, 2, 3]);
	}

	#[test]
	fn full_mut() {
		let mut arr = [1, 2, 3_i8];
		assert_eq!(arr.sub_array_mut::<3>(0), &mut [1, 2, 3]);
	}

	#[test]
	fn first_ref() {
		let arr = [1, 2, 3_u16];
		assert_eq!(arr.sub_array_ref::<1>(0), &[1]);
	}

	#[test]
	fn first_mut() {
		let mut arr = [1, 2, 3_u16];
		assert_eq!(arr.sub_array_mut::<1>(0), &mut [1]);
	}

	#[test]
	fn middle_ref() {
		let arr = [1, 2, 3_i16];
		assert_eq!(arr.sub_array_ref::<1>(1), &[2]);
	}

	#[test]
	fn middle_mut() {
		let mut arr = [1, 2, 3_i16];
		assert_eq!(arr.sub_array_mut::<1>(1), &mut [2]);
	}

	#[test]
	fn last_ref() {
		let arr = [1, 2, 3_i16];
		assert_eq!(arr.sub_array_ref::<1>(2), &[3]);
	}

	#[test]
	fn last_mut() {
		let mut arr = [1, 2, 3_i16];
		assert_eq!(arr.sub_array_mut::<1>(2), &mut [3]);
	}

	#[derive(Debug, PartialEq, Eq)]
	struct NotClone(&'static str);

	const NOT_CLONE_ARRAY: [NotClone; 5] = [
		NotClone("abc"),
		NotClone("foo"),
		NotClone("bar"),
		NotClone("qux"),
		NotClone("fox"),
	];

	#[test]
	fn not_clone_ref() {
		let exp_arr = [NotClone("foo"), NotClone("bar"), NotClone("qux")];
		let arr = NOT_CLONE_ARRAY;
		assert_eq!(arr.sub_array_ref::<3>(1), &exp_arr);
	}

	#[test]
	fn not_clone_mut() {
		let mut exp_arr = [NotClone("foo"), NotClone("bar"), NotClone("qux")];
		let mut arr = NOT_CLONE_ARRAY;
		assert_eq!(arr.sub_array_mut::<3>(1), &mut exp_arr);
	}

	#[test]
	fn some_strings() {
		let arr: [String; 5] = NOT_CLONE_ARRAY.map(|s| s.0.to_string());
		assert_eq!(
			arr.sub_array_ref::<2>(2),
			&[String::from("bar"), String::from("qux")]
		);
	}
}