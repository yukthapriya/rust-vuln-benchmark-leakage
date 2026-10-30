// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0080
// CVE: N/A
// CWE: N/A
// Crate: parity-util-mem
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/parity-util-mem
// Extraction Method: repo_search
// GitHub Repo: paritytech/parity-common
// Commit: cb633d1
// File: bounded-collections/src/bounded_btree_map.rs


/// Gets a mutable iterator over the entries of the map, sorted by key.
 	///
 	/// See [`BTreeMap::iter_mut`] for more information.
	pub fn iter_mut(&mut self) -> alloc::collections::btree_map::IterMut<K, V> {
 		self.0.iter_mut()
 	}