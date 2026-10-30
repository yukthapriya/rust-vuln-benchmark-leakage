// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0041 (FIXED)
// Crate: sized-chunks
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: bodil/sized-chunks
// Commit: b3ebeb0
// File: src/inline_array/mod.rs


pub struct InlineArray<A, T> {
     // Alignment tricks
     //
    // We need both the `_header_align` and `data` to be properly aligned in memory. We do a few tricks
     // to handle that.
     //
     // * An alignment is always power of 2. Therefore, with a pair of alignments, one is always
     //   a multiple of the other (one way or the other).
     // * A struct is aligned to at least the max alignment of each of its fields.
    // * A `repr(C)` struct follows the order of fields and pushes each as close to the previous one
     //   as allowed by alignment.
     //
     // By placing two "fake" fields that have 0 size, but an alignment first, we make sure that all
             self_.data.as_ptr() as usize,
             "Padding at the start of struct",
         );
         assert_eq!(
             self_.data.as_ptr() as usize % mem::align_of::<usize>(),
             0,
             "Unaligned header"
         );
        assert!(mem::size_of::<Self>() == mem::size_of::<T>() || mem::align_of::<T>() < mem::align_of::<Self>());
        assert_eq!(0, unsafe { self_.data() } as usize % mem::align_of::<A>());
        assert_eq!(0, unsafe { self_.data_mut() } as usize % mem::align_of::<A>());
         assert!(Self::ELEMENT_SKIP == 0 || Self::HEADER_SKIP == 0);
         unsafe { ptr::write(self_.len_mut(), 0usize) };
         self_
         assert_eq!(chunk[0], "Hello");
     }
 
    #[test]
    fn float_align() {
        let mut chunk: InlineArray<f64, [u8; 16]> = InlineArray::new();
        chunk.push(1234.);
        assert_eq!(chunk[0], 1234.);

        let mut chunk: InlineArray<f64, [u8; 17]> = InlineArray::new();
        chunk.push(1234.);
        assert_eq!(chunk[0], 1234.);
    }

     #[test]
     fn recursive_types_compile() {
         #[allow(dead_code)]