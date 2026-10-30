// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0035
// CVE: CVE-2021-28036
// CWE: N/A
// Crate: quinn
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/quinn
// Extraction Method: repo_search
// GitHub Repo: quinn-rs/quinn
// Commit: 0a6635d
// File: quinn-proto/src/connection/pacing.rs


let pace_duration = Duration::from_nanos((TARGET_BURST_INTERVAL.as_nanos() * 4 / 5) as u64);
 
        assert_eq!(
            pacer
                .delay(rtt, mtu as u64, mtu, window, old_instant)
                .expect("Send must be delayed")
                .duration_since(old_instant),
            pace_duration
        );
 
         // Refill half of the tokens
         assert_eq!(
             pacer.delay(