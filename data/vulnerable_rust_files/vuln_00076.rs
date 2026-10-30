// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0041
// CVE: CVE-2020-25791
// CWE: N/A
// Crate: sized-chunks
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/sized-chunks
// Extraction Method: repo_search
// GitHub Repo: bodil/sized-chunks
// Commit: b3ebeb0
// File: src/inline_array/mod.rs


pub struct InlineArray<A, T> {
     // Alignment tricks
     //
    // We need both the usize header and data to be properly aligned in memory. We do a few tricks
     // to handle that.
     //
     // * An alignment is always power of 2. Therefore, with a pair of alignments, one is always
     //   a multiple of the other (one way or the other).
     // * A struct is aligned to at least the max alignment of each of its fields.
    // * A repr(C) struct follows the order of fields and pushes each as close to the previous one
     //   as allowed by alignment.
     //
     // By placing two "fake" fields that have 0 size, but an alignment first, we make sure that all
             self_.data.as_ptr() as usize,
             "Padding at the start of struct",
         );
        assert_eq!(mem::size_of::<Self>(), mem::size_of::<T>());
         assert_eq!(
             self_.data.as_ptr() as usize % mem::align_of::<usize>(),
             0,
             "Unaligned header"
         );
        assert_eq!(
            self_.data.as_ptr() as usize % mem::align_of::<A>(),
            0,
            "Unaligned elements"
        );
         assert!(Self::ELEMENT_SKIP == 0 || Self::HEADER_SKIP == 0);
         unsafe { ptr::write(self_.len_mut(), 0usize) };
         self_
         assert_eq!(chunk[0], "Hello");
     }
 
     #[test]
     fn recursive_types_compile() {
         #[allow(dead_code)]