// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0028
// CVE: N/A
// CWE: N/A
// Crate: cve-rs
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/cve-rs
// Extraction Method: repo_search
// GitHub Repo: Speykious/cve-rs
// Commit: 80d3a0e
// File: src/lifetime_expansion.rs


///
 /// Safety? What's that?
 pub fn expand<'a, 'b, T: ?Sized>(x: &'a T) -> &'b T {
	let f: fn(_, &'a T) -> &'b T = lifetime_translator;
 	f(STATIC_UNIT, x)
 }
 
 ///
 /// Safety? What's that?
 pub fn expand_mut<'a, 'b, T: ?Sized>(x: &'a mut T) -> &'b mut T {
	let f: fn(_, &'a mut T) -> &'b mut T = lifetime_translator_mut;
 	f(STATIC_UNIT, x)
 }