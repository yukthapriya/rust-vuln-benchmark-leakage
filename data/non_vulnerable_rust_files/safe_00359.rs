// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0055 (FIXED)
// Crate: lexical
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Alexhuszagh/rust-lexical
// Commit: 57dfd21
// File: extras/benchmark/write-float/special.rs


let value = fastrand::$i($exp_mask..);
             // NOTE: We want mem::transmute, not from_bits because we
             // don't want the special handling of from_bits
            // FIXME: Move to `to_bits` in 1.83.0+ (const stable)
            // FIXME: change to `unnecessary_transmutes` when supported.
            #[allow(
                renamed_and_removed_lints,
                unknown_lints,
                clippy::transmute_int_to_float,
                unnecessary_transmutes
            )]
             vec.push(unsafe { mem::transmute::<$i, $f>(value) });
         }
         vec