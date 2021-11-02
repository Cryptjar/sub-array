#![no_std]
//
// This crate is entirely safe (tho that's not a guarantee for the future)
#![forbid(unsafe_code)]

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
//!
//! // Get a sub-array starting at offset 1
//! let sub: &[u8; 3] = arr.sub_array_ref(1);
//! assert_eq!(sub, &[2, 3, 4]);
//! ```
//!
//! Initializing an `[u8;10]` array with `(u16, u32, u32)`:
//!
//! ```
//! use sub_array::SubArray;
//!
//! let foo: u16 = 42;
//! let bar: u32 = 0x1234;
//! let baz: u32 = 0x5678;
//!
//! let mut arr = [0_u8; 10];
//! *arr.sub_array_mut::<2>(0) = foo.to_be_bytes();
//! *arr.sub_array_mut::<4>(2) = bar.to_be_bytes();
//! *arr.sub_array_mut::<4>(6) = baz.to_be_bytes();
//!
//! assert_eq!(
//!     arr,
//!     [
//!         0, 42, // foo
//!         0x0, 0x0, 0x12, 0x34, // bar
//!         0x0, 0x0, 0x56, 0x78, // baz
//!     ]
//! );
//! ```


/// Array that can be slice into a smaller sub-array
///
/// Also see the [crate] level reference.
pub trait SubArray {
	/// The value type of this array.
	///
	/// This is the `T` in `[T; N]` on regular arrays.
	type Item;

	/// Get a reference to a sub-array of length `N` starting at `offset`.
	///
	/// # Panics
	/// Panics if `offset + N` exceeds the length of this array.
	///
	/// # Example
	/// ```
	/// use sub_array::SubArray;
	///
	/// let arr: [u8; 5] = [9, 8, 7, 6, 5];
	///
	/// // Get a sub-array starting at offset 3
	/// let sub: &[u8; 2] = arr.sub_array_ref(3);
	/// assert_eq!(sub, &[6, 5]);
	/// ```
	fn sub_array_ref<const N: usize>(&self, offset: usize) -> &[Self::Item; N];

	/// Get a mutable reference to a sub-array of length `N` starting at
	/// `offset`.
	///
	/// # Panics
	/// Panics if `offset + N` exceeds the length of this array.
	///
	/// # Example
	/// ```
	/// use sub_array::SubArray;
	///
	/// let mut arr: [u8; 5] = [9, 8, 7, 6, 5];
	///
	/// // Get a mutable sub-array starting at offset 0
	/// let sub: &mut [u8; 2] = arr.sub_array_mut(0);
	/// assert_eq!(sub, &mut [9, 8]);
	/// ```
	fn sub_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [Self::Item; N];
}

/// Implementation on regular arrays
impl<T, const M: usize> SubArray for [T; M] {
	type Item = T;

	fn sub_array_ref<const N: usize>(&self, offset: usize) -> &[Self::Item; N] {
		self[offset..(offset + N)].try_into().unwrap()
	}

	fn sub_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [Self::Item; N] {
		(&mut self[offset..(offset + N)]).try_into().unwrap()
	}
}

/// Implementation on slices
impl<T> SubArray for [T] {
	type Item = T;

	fn sub_array_ref<const N: usize>(&self, offset: usize) -> &[Self::Item; N] {
		self[offset..(offset + N)].try_into().unwrap()
	}

	fn sub_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [Self::Item; N] {
		(&mut self[offset..(offset + N)]).try_into().unwrap()
	}
}

/// Implementation on mutable references
impl<T> SubArray for &mut T
where
	T: SubArray,
{
	type Item = T::Item;

	fn sub_array_ref<const N: usize>(&self, offset: usize) -> &[Self::Item; N] {
		(**self).sub_array_ref(offset)
	}

	fn sub_array_mut<const N: usize>(&mut self, offset: usize) -> &mut [Self::Item; N] {
		(**self).sub_array_mut(offset)
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

	fn test_by_slice(s: &[u8]) -> &[u8; 3] {
		s.sub_array_ref(4)
	}

	#[test]
	fn slices() {
		let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9_u8];

		let slice: &[u8] = &arr;

		let arr_ref = test_by_slice(slice);

		assert_eq!(arr_ref, &[5, 6, 7]);
		assert_eq!(arr_ref, arr.sub_array_ref(4));
		assert_eq!(arr_ref, &slice[4..7]);
	}
}
