// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0084 (FIXED)
// Crate: hpack
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mlalic/hpack-rs
// Commit: 921ba85
// File: src/decoder.rs


let header_list = decoder.decode(&[0x82]).ok().unwrap();
 
        assert_eq!(vec![(b":method".to_vec(), b"GET".to_vec())], header_list);
     }
 
     #[test]