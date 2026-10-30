// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0081
// CVE: N/A
// CWE: N/A
// Crate: safemem
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/safemem
// Extraction Method: repo_search
// GitHub Repo: abonander/safemem
// Commit: 76c568e
// File: src/lib.rs


len_check!(slice, src_idx, len);
     len_check!(slice, dest_idx, len);
 
     let ptr = slice.as_mut_ptr();
 
     unsafe {
 
 /// Prepend `elems` to `vec`, resizing if necessary.
 ///
/// ###Panics
 /// If `vec.len() + elems.len()` overflows.
 #[cfg(feature = "std")]
 pub fn prepend<T: Copy>(elems: &[T], vec: &mut Vec<T>) {
    // Our overflow check occurs here, no need to do it ourselves.
    vec.reserve(elems.len());

    let old_len = vec.len();
    let new_len = old_len + elems.len();

    unsafe {
        vec.set_len(new_len);
     }

    // Move the old elements down to the end.
    if old_len > 0 {
        copy_over(vec, 0, elems.len(), old_len);
    }

    vec[..elems.len()].copy_from_slice(elems);
 }
 
 #[cfg(test)]
         prepend(&[1, 2], &mut vec);
         assert_eq!(vec, &[1, 2, 3, 4, 5]);
     }
 }