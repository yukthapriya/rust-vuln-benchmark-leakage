// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0088 (FIXED)
// Crate: loopdev
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mdaffin/loopdev
// Commit: c9f91e8
// File: tests/integration_test.rs


#[test]
 fn add_a_loop_device() {
     let _lock = setup();
 
     let lc = LoopControl::open().expect("should be able to open the LoopControl device");
    assert!(lc.add(1).is_ok());
    assert!(lc.add(1).is_err());
 }