// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0084
// CVE: N/A
// CWE: N/A
// Crate: hpack
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/hpack
// Extraction Method: repo_search
// GitHub Repo: mlalic/hpack-rs
// Commit: 921ba85
// File: src/decoder.rs


let header_list = decoder.decode(&[0x82]).ok().unwrap();
 
        assert_eq!([(b":method".to_vec(), b"GET".to_vec())], header_list);
     }
 
     #[test]