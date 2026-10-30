// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0354 (FIXED)
// Crate: vodozemac
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: matrix-org/vodozemac
// Commit: e5f438d
// File: src/olm/account/mod.rs


// one-time key type that takes `self`. If we didn't do this,
             // someone could maliciously pretend to use up our one-time key and
             // make us drop the private part. Unsuspecting users that actually
            // try to use such an one-time key won't be able to communicate with
             // us. This is strictly worse than the one-time key exhaustion
             // scenario.
             self.remove_one_time_key_helper(pre_key_message.one_time_key());