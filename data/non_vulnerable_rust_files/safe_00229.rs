// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0103 (FIXED)
// Crate: molecule
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: nervosnetwork/molecule
// Commit: 0821b7a
// File: bindings/rust/src/lazy_reader.rs


/// Assuming a cursor is `fixvec`, return raw data without header.
     pub fn fixvec_slice_raw_bytes(&self) -> Result<Cursor, Error> {
        self.slice_by_offset(NUMBER_SIZE, self.unpack_number()?)
     }
 
     /// helper function for generated code