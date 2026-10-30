// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0130 (FIXED)
// Crate: lru
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: jeromefroe/lru-rs
// Commit: 1b21bf1
// File: src/lib.rs


S: BuildHasher + Clone,
 {
     fn clone(&self) -> Self {
        let map_cap = if self.is_unbounded() {
            self.len()
        } else {
            self.cap().get()
        };
         let mut new_lru = LruCache::construct(
             self.cap(),
            HashMap::with_capacity_and_hasher(map_cap, self.map.hasher().clone()),
         );
 
         for (key, value) in self.iter().rev() {
         cache
     }
 
    /// Whether this LRU cache is unbounded.
    fn is_unbounded(&self) -> bool {
        self.cap() == NonZeroUsize::MAX
    }

     /// Puts a key-value pair into cache. If the key already exists in the cache, then it updates
     /// the key's value and returns the old value. Otherwise, `None` is returned.
     ///
         assert_eq!(cache.pop_lru(), None);
         assert_eq!(cloned.pop_lru(), None);
     }

    #[test]
    fn test_clone_unbounded() {
        let mut cache = LruCache::unbounded();
        cache.put("a", 1);
        cache.put("b", 2);
        cache.put("c", 3);

        let mut cloned = cache.clone();

        assert_eq!(cache.pop_lru(), Some(("a", 1)));
        assert_eq!(cloned.pop_lru(), Some(("a", 1)));

        assert_eq!(cache.pop_lru(), Some(("b", 2)));
        assert_eq!(cloned.pop_lru(), Some(("b", 2)));

        assert_eq!(cache.pop_lru(), Some(("c", 3)));
        assert_eq!(cloned.pop_lru(), Some(("c", 3)));

        assert_eq!(cache.pop_lru(), None);
        assert_eq!(cloned.pop_lru(), None);
    }
 }
 
 /// Doctests for what should *not* compile