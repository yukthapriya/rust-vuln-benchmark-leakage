// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0056 (FIXED)
// Crate: vm-memory
// Type: memory_safety (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: rust-vmm/vm-memory
// Commit: aff1dd4
// File: src/volatile_memory.rs


/// Returns a [`VolatileSlice`](struct.VolatileSlice.html) of `count` bytes starting at
     /// `offset`.
    ///
    /// Note that the property `get_slice(offset, count).len() == count` MUST NOT be
    /// relied on for the correctness of unsafe code. This is a safe function inside of a
    /// safe trait, and implementors are under no obligation to follow its documentation.
     fn get_slice(&self, offset: usize, count: usize) -> Result<VolatileSlice<BS<Self::B>>>;
 
     /// Gets a slice of memory for the entire region that supports volatile access.
     /// Gets a `VolatileRef` at `offset`.
     fn get_ref<T: ByteValued>(&self, offset: usize) -> Result<VolatileRef<T, BS<Self::B>>> {
         let slice = self.get_slice(offset, size_of::<T>())?;

        assert_eq!(
            slice.len(),
            size_of::<T>(),
            "VolatileMemory::get_slice(offset, count) returned slice of length != count."
        );

        // SAFETY: This is safe because the invariants of the constructors of VolatileSlice ensure that
        // slice.addr is valid memory of size slice.len(). The assert above ensures that
        // the length of the slice is exactly enough to hold one `T`. Lastly, the lifetime of the
        // returned VolatileRef match that of the VolatileSlice returned by get_slice and thus the
        // lifetime one `self`.
         unsafe {
             Ok(VolatileRef::with_bitmap(
                 slice.addr,
                 size: size_of::<T>(),
             })?;
         let slice = self.get_slice(offset, nbytes as usize)?;

        assert_eq!(
            slice.len(),
            nbytes as usize,
            "VolatileMemory::get_slice(offset, count) returned slice of length != count."
        );

        // SAFETY: This is safe because the invariants of the constructors of VolatileSlice ensure that
        // slice.addr is valid memory of size slice.len(). The assert above ensures that
        // the length of the slice is exactly enough to hold `n` instances of `T`. Lastly, the lifetime of the
        // returned VolatileArrayRef match that of the VolatileSlice returned by get_slice and thus the
        // lifetime one `self`.
         unsafe {
             Ok(VolatileArrayRef::with_bitmap(
                 slice.addr,
     unsafe fn aligned_as_ref<T: ByteValued>(&self, offset: usize) -> Result<&T> {
         let slice = self.get_slice(offset, size_of::<T>())?;
         slice.check_alignment(align_of::<T>())?;

        assert_eq!(
            slice.len(),
            size_of::<T>(),
            "VolatileMemory::get_slice(offset, count) returned slice of length != count."
        );

        // SAFETY: This is safe because the invariants of the constructors of VolatileSlice ensure that
        // slice.addr is valid memory of size slice.len(). The assert above ensures that
        // the length of the slice is exactly enough to hold one `T`.
        // Dereferencing the pointer is safe because we check the alignment above, and the invariants
        // of this function ensure that no aliasing pointers exist. Lastly, the lifetime of the
        // returned VolatileArrayRef match that of the VolatileSlice returned by get_slice and thus the
        // lifetime one `self`.
        unsafe { Ok(&*(slice.addr as *const T)) }
     }
 
     /// Returns a mutable reference to an instance of `T` at `offset`. Mutable accesses performed
         let slice = self.get_slice(offset, size_of::<T>())?;
         slice.check_alignment(align_of::<T>())?;
 
        assert_eq!(
            slice.len(),
            size_of::<T>(),
            "VolatileMemory::get_slice(offset, count) returned slice of length != count."
        );

        // SAFETY: This is safe because the invariants of the constructors of VolatileSlice ensure that
        // slice.addr is valid memory of size slice.len(). The assert above ensures that
        // the length of the slice is exactly enough to hold one `T`.
        // Dereferencing the pointer is safe because we check the alignment above, and the invariants
        // of this function ensure that no aliasing pointers exist. Lastly, the lifetime of the
        // returned VolatileArrayRef match that of the VolatileSlice returned by get_slice and thus the
        // lifetime one `self`.

        unsafe { Ok(&mut *(slice.addr as *mut T)) }
     }
 
     /// Returns a reference to an instance of `T` at `offset`. Mutable accesses performed
         let slice = self.get_slice(offset, size_of::<T>())?;
         slice.check_alignment(align_of::<T>())?;
 
        assert_eq!(
            slice.len(),
            size_of::<T>(),
            "VolatileMemory::get_slice(offset, count) returned slice of length != count."
        );

        // SAFETY: This is safe because the invariants of the constructors of VolatileSlice ensure that
        // slice.addr is valid memory of size slice.len(). The assert above ensures that
        // the length of the slice is exactly enough to hold one `T`.
        // Dereferencing the pointer is safe because we check the alignment above. Lastly, the lifetime of the
        // returned VolatileArrayRef match that of the VolatileSlice returned by get_slice and thus the
        // lifetime one `self`.
         unsafe { Ok(&*(slice.addr as *const T)) }
     }