// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0080
// CVE: CVE-2023-53156
// CWE: N/A
// Crate: transpose
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/transpose
// Extraction Method: repo_search
// GitHub Repo: ejmahler/transpose
// Commit: c4bcd39
// File: src/in_place.rs


/// 
 /// Panics if `input.len() != input_width * input_height` or if `output.len() != input_width * input_height`
 pub fn transpose_inplace<T: Copy>(buffer: &mut [T], scratch: &mut [T], width: usize, height: usize) {
	assert_eq!(width*height, buffer.len());
 	assert_eq!(core::cmp::max(width, height), scratch.len());
 
 	let gcd = StrengthReducedUsize::new(num_integer::gcd(width, height));