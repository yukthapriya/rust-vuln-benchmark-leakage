// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0088
// CVE: N/A
// CWE: N/A
// Crate: loopdev
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/loopdev
// Extraction Method: repo_search
// GitHub Repo: mdaffin/loopdev
// Commit: c9f91e8
// File: tests/integration_test.rs


#[test]
 fn add_a_loop_device() {
     let _lock = setup();
    let dev = std::fs::read_dir("/dev").expect("should be able to open /dev");
    let device_count = dev
        .map(|r| {
            r.as_ref()
                .expect("readdir failed")
                .file_name()
                .to_string_lossy()
                .to_string()
        })
        .filter(|r| r != "loop-control")
        .filter(|r| r.contains("loop"))
        .count();
 
     let lc = LoopControl::open().expect("should be able to open the LoopControl device");
    assert!(lc.add(device_count as u32).is_err()); // EEXIST
    assert!(lc.add(device_count as u32 + 1).is_ok())
 }