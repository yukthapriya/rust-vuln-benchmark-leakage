fn _get<Q>(&'a self, key: &Q) -> Option<Ref<'a, K, V, S>>
    where
        K: Borrow<Q>,
        Q: Hash + Eq + ?Sized,
    {
        let hash = self.hash_usize(&key);

        let idx = self.determine_shard(hash);

        let shard = unsafe { self._yield_read_shard(idx) };

        if let Some((kptr, vptr)) = shard.get_key_value(key) {
            unsafe {
                let kptr = util::change_lifetime_const(kptr);

                let vptr = util::change_lifetime_const(vptr);

                Some(Ref::new(shard, kptr, vptr.get()))
            }
        } else {
            None
        }
    }