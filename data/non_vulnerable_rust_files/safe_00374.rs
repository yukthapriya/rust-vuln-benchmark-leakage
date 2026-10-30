// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0080 (FIXED)
// Crate: transpose
// Type: overflow (FIXED)
// Extraction Method: repo_search
// GitHub Repo: ejmahler/transpose
// Commit: c4bcd39
// File: src/in_place.rs


/// 
 /// Panics if `input.len() != input_width * input_height` or if `output.len() != input_width * input_height`
 pub fn transpose_inplace<T: Copy>(buffer: &mut [T], scratch: &mut [T], width: usize, height: usize) {
	assert_eq!(width.checked_mul(height), Some(buffer.len()));
 	assert_eq!(core::cmp::max(width, height), scratch.len());
 
 	let gcd = StrengthReducedUsize::new(num_integer::gcd(width, height));