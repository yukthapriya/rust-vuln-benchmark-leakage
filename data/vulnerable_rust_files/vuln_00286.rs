// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0040
// CVE: N/A
// CWE: N/A
// Crate: owning_ref
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/owning_ref
// Extraction Method: repo_search
// GitHub Repo: noamtashma/owning-ref-unsoundness
// Commit: daa00f4
// File: src/main.rs


let mut ow_ref = OwningRefMut::new(Box::new(5));
     *ow_ref.as_owner_mut() = Box::new(9);
     println!("Reading deallocated memory: {}", *ow_ref);
    println!("");
 }
 
 fn unstable_address() {
     println!("unstable_address");
     let ow_ref = OwningRef::new(Box::new(5));
     let new_ow_ref = ow_ref.map_with_owner(|owner, _my_ref| owner);
     println!("Reading memory that was moved from: {}", *new_ow_ref);
    println!("");
 }
 
 // Original credit: [https://github.com/comex/owning_ref_bug/blob/master/src/main.rs]
     // We could call `as_owner_mut`, but we already saw that `as_owner_mut` is inherently unsound.
     *new_ow_ref.as_owner().borrow_mut() = Box::new(9);
     println!("Reading deallocated memory: {}", *new_ow_ref);
    println!("");
 }
 
 fn ref_mut_to_ref() {
     // instead of `OwningRefMut::as_owner`.
     *new_ow_ref.as_owner().borrow_mut() = Box::new(9);
     println!("Reading deallocated memory: {}", *new_ow_ref);
    println!("");
 }
 
 fn ref_mut_to_ref_2() {
         println!("Second read of reference {}", x);
         x
     });
    println!("");
 }
 
 // Credit: [https://github.com/Kimundi/owning-ref-rs/issues/71]
 fn tricky_map_unsoundness<'a, T>(input: &'a T) -> &'static T {
     let ow_ref1 = OwningRef::new(Box::new(()));
     let input_ref_ref = &input;
    let ow_ref2: OwningRef<Box<()>, &&T> = ow_ref1.map(|x| &input_ref_ref);
     let ow_ref3: OwningRef<Box<()>, T> = ow_ref2.map(|s| &***s);
     &*Box::leak(Box::new(ow_ref3))
 }