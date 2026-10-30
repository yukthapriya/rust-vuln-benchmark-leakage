// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0122 (FIXED)
// Crate: beef
// Type: race_condition (FIXED)
// Extraction Method: repo_search
// GitHub Repo: maciejhirsz/beef
// Commit: 6bb5597
// File: src/traits.rs


U: Capacity,
         {
             // Convert to `String::into_raw_parts` once stabilized
            // We need to go through Vec here to get provenance for the entire allocation
            // instead of just the initialized parts.
            let mut owned = ManuallyDrop::new(owned.into_bytes());
             let (fat, cap) = U::store(owned.len(), owned.capacity());
 
             (