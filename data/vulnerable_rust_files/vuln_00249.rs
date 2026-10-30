// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0130
// CVE: CVE-2021-45720
// CWE: N/A
// Crate: lru
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/lru
// Extraction Method: repo_search
// GitHub Repo: jeromefroe/lru-rs
// Commit: 1b21bf1
// File: src/lib.rs


S: BuildHasher + Clone,
 {
     fn clone(&self) -> Self {
         let mut new_lru = LruCache::construct(
             self.cap(),
            HashMap::with_capacity_and_hasher(self.cap().get(), self.map.hasher().clone()),
         );
 
         for (key, value) in self.iter().rev() {
         cache
     }
 
     /// Puts a key-value pair into cache. If the key already exists in the cache, then it updates
     /// the key's value and returns the old value. Otherwise, `None` is returned.
     ///
         assert_eq!(cache.pop_lru(), None);
         assert_eq!(cloned.pop_lru(), None);
     }
 }
 
 /// Doctests for what should *not* compile