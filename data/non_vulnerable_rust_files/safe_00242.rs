// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0120 (FIXED)
// Crate: abomonation
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: TimelyDataflow/abomonation
// Commit: 40ba782
// File: src/lib.rs


pub unsafe fn encode<T: Abomonation>(typed: &T, bytes: &mut Vec<u8>) {
     let start = bytes.len();            // may not be empty!
     let slice = std::slice::from_raw_parts(mem::transmute(typed), mem::size_of::<T>());

    // TODO : At the moment, this does not compile to a memcpy, 
    // due to Rust working around LLVM soundness bugs.

    // bytes.write_all(slice).unwrap();    // Rust claims a write to a Vec<u8> will never fail.

    // TODO : Instead, we use the following hunk of code:
    let position = bytes.len();
    bytes.reserve(slice.len());
    ::std::ptr::copy_nonoverlapping(slice.as_ptr(), bytes.as_mut_ptr().offset(position as isize), slice.len());
    bytes.set_len(position + slice.len());
    // TODO : End hunk of replacement code

     let result: &mut T = mem::transmute(bytes.as_mut_ptr().offset(start as isize));
     result.embalm();
     typed.entomb(bytes);
         ::std::ptr::copy_nonoverlapping(typed_bytes.as_ptr(), bytes.as_mut_ptr().offset(position as isize), typed_bytes.len());
         bytes.set_len(position + typed_bytes.len());
         // TODO : End hunk of replacement code

         for element in bytes_to_typed::<T>(&mut bytes[position..], self.len()) { element.embalm(); }
         for element in self.iter() { element.entomb(bytes); }
     }