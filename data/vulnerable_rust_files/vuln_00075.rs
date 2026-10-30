// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0040
// CVE: CVE-2020-35894
// CWE: N/A
// Crate: obstack
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/obstack
// Extraction Method: repo_search
// GitHub Repo: petertodd/rust-obstack
// Commit: 291fca7
// File: src/lib.rs


let start_ptr = self.tip.as_mut_ptr()
                                 .offset(self.tip.len_items() as isize);
 
        let padding = start_ptr as usize % alignment;

        debug_assert!(padding < alignment);
        debug_assert_eq!(padding, 0);

         let start_ptr = start_ptr.offset(AlignedVec::<A>::bytes_to_items(padding) as isize);

        let new_used = self.tip.len_items() + padding + AlignedVec::<A>::bytes_to_items(size);
 
         if new_used <= self.tip.capacity_items() {
             self.tip.set_len_items(new_used);