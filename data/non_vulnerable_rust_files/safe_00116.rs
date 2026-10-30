// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0103 (FIXED)
// Crate: autorand
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mersinvald/autorand-rs
// Commit: 39f9eae
// File: src/lib.rs


$(
         impl<T: Random> Random for [T; $s] {
             fn random() -> Self {
                use std::mem::{MaybeUninit, transmute_copy, size_of};
                 unsafe {
                    let mut array: [MaybeUninit<T>; $s] = MaybeUninit::uninit().assume_init();
                    for elem in &mut array[..] {
                        *elem = MaybeUninit::new(T::random());
                     }

                    // See https://github.com/rust-lang/rust/issues/47966
                    debug_assert!(size_of::<[MaybeUninit<T>; $s]>() == size_of::<[T; $s]>());
                    transmute_copy::<_, [T; $s]>(&array)
                 }
             }
         }