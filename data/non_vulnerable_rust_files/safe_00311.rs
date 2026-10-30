// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0081 (FIXED)
// Crate: json
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: maciejhirsz/json-rust
// Commit: 752f942
// File: src/lib.rs


// Construct the actual array
     (@END $( $i:expr, )*) => ({
        let size = 0 $( + {let _ = &$i; 1} )*;
         let mut array = Vec::with_capacity(size);
 
         $(
 
     // Construct the actual object
     (@END $( $k:expr => $v:expr, )*) => ({
        let size = 0 $( + {let _ = &$k; 1} )*;
         let mut object = $crate::object::Object::with_capacity(size);
 
         $(