// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0035 (FIXED)
// Crate: quinn
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: quinn-rs/quinn
// Commit: 0a6635d
// File: quinn-proto/src/connection/pacing.rs


let pace_duration = Duration::from_nanos((TARGET_BURST_INTERVAL.as_nanos() * 4 / 5) as u64);
 
        let actual_delay = pacer
            .delay(rtt, mtu as u64, mtu, window, old_instant)
            .expect("Send must be delayed")
            .duration_since(old_instant);

        let diff = actual_delay.abs_diff(pace_duration);
 
        // Allow up to 2ns difference due to rounding
        assert!(
            diff < Duration::from_nanos(2),
            "expected ≈ {pace_duration:?}, got {actual_delay:?} (diff {diff:?})"
        );
         // Refill half of the tokens
         assert_eq!(
             pacer.delay(