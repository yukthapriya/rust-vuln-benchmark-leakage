// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0081 (FIXED)
// Crate: safemem
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: abonander/safemem
// Commit: 76c568e
// File: src/lib.rs


len_check!(slice, src_idx, len);
     len_check!(slice, dest_idx, len);
 
    // At any point a Rust reference exists, the compiler is free to do this.
    // So we explicitely add it to be caught by miri.
    #[cfg(miri)]
    slice.iter().copied().for_each(drop);

     let ptr = slice.as_mut_ptr();
 
     unsafe {
 
 /// Prepend `elems` to `vec`, resizing if necessary.
 ///
/// ### Panics
///
 /// If `vec.len() + elems.len()` overflows.
 #[cfg(feature = "std")]
 pub fn prepend<T: Copy>(elems: &[T], vec: &mut Vec<T>) {
    let elems_len = elems.len(); // `<= isize::MAX as usize`
    if elems_len == 0 { return; }

    let old_len = vec.len(); // `<= isize::MAX as usize`
    if old_len == 0 {
        // Prepend = append: delegate to Rust's stdlib implementation.
        vec.extend_from_slice(elems);
    } else {
        // Our overflow check occurs here, no need to do it ourselves.
        vec.reserve(elems_len);
        let ptr = vec.as_mut_ptr();
        unsafe {
            // Move the old elements down to the end.
            ptr::copy(
                ptr,
                ptr.offset(elems_len as isize),
                old_len,
            );
            // Copy the input elements to the start
            ptr::copy_nonoverlapping(
                elems.as_ptr(),
                ptr,
                elems_len,
            );
            // Set the len *after* having initialized the elements.
            vec.set_len(old_len + elems_len);
        }
     }
 }
 
 #[cfg(test)]
         prepend(&[1, 2], &mut vec);
         assert_eq!(vec, &[1, 2, 3, 4, 5]);
     }

    /// Detect potential uninit values when running miri
    #[test]
    #[cfg(all(
        feature = "std",
        miri,
    ))]
    fn prepend_bool() {
        prepend(&[true], &mut vec![false]);
    }
 }