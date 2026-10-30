pub fn downgrade(self) -> Ref<'a, K, V, S> {
        unsafe {
            Ref::new(
                parking_lot::RwLockWriteGuard::downgrade(self.guard),
                self.k,
                self.v,
            )
        }
    }