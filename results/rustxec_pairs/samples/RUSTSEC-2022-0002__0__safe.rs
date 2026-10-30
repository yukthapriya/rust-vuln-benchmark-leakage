fn _entry(&'a self, key: K) -> Entry<'a, K, V, S> {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let shard = unsafe { self._yield_write_shard(idx) };

        if let Some((kptr, vptr)) = shard.get_key_value(&key) {
            unsafe {
                let kptr: *const K = kptr;
                let vptr: *mut V = vptr.as_ptr();
                Entry::Occupied(OccupiedEntry::new(shard, key, (kptr, vptr)))
            }
        } else {
            unsafe { Entry::Vacant(VacantEntry::new(shard, key)) }
        }
    }