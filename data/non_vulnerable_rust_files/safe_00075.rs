// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0040 (FIXED)
// Crate: obstack
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: petertodd/rust-obstack
// Commit: 291fca7
// File: src/lib.rs


let start_ptr = self.tip.as_mut_ptr()
                                 .offset(self.tip.len_items() as isize);
 
        let padding = (alignment - (start_ptr as usize % alignment)) % alignment;
         let start_ptr = start_ptr.offset(AlignedVec::<A>::bytes_to_items(padding) as isize);
        let new_used = self.tip.len_items() + AlignedVec::<A>::bytes_to_items(padding + size);
 
         if new_used <= self.tip.capacity_items() {
             self.tip.set_len_items(new_used);