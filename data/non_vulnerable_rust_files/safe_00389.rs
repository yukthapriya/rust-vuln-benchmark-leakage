// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0004 (FIXED)
// Crate: cosmwasm
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: CosmWasm/cosmwasm
// Commit: f4b890e
// File: packages/std/src/math/decimal.rs


self.0 / Self::DECIMAL_FRACTIONAL
     }
 
    /// Converts this decimal to an unsigned integer by rounding up
     /// to the next integer, e.g. 22.3 becomes 23.
     ///
     /// ## Examples