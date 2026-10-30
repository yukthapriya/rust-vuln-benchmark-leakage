// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0018 (FIXED)
// Crate: qwutils
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: qwertz19281/rust_utils
// Commit: 833f581
// File: src/imp/vec.rs


a.insert_slice_clone(2,&b);
     assert_eq!(&a,&[1,2,3,4,5,6,7,8]);
     assert_eq!(a.len(),8);
}