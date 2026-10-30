// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2025-0107
// CVE: N/A
// CWE: N/A
// Crate: borrowck_sacrifices
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/borrowck_sacrifices
// Extraction Method: repo_search
// GitHub Repo: alexpyattaev/borrowck_sacrifices
// Commit: 1bdaa14
// File: src/unsafe_casts.rs


/// Utility function to cast any sized type into slice of bytes
/// This is mostly for debugging purposes
 /// # Safety
/// This is safe since returned slice is readonly (as long as you do not modify thing it is pointing into), and returned slice is lifetime-bound to
 /// the original reference.
 /// # Examples
 /// Let us see how 42 is represented:
 /// ```
 /// # use borrowck_sacrifices::unsafe_casts::any_as_u8_slice;
 /// let thing = 42i32;
/// let u8slice = any_as_u8_slice(&thing);
 /// println!("{:?}", u8slice);
 /// ```
 /// Make sure we can not shoot ourselves in the foot:
 /// ```compile_fail
 /// # use borrowck_sacrifices::unsafe_casts::any_as_u8_slice;
/// 
 /// let u8slice = {
 ///    let thing = 42i32;// this variable exists only in inner scope
 ///    any_as_u8_slice(&thing);
 ///    }
/// 
 /// println!("{:?}", u8slice); //accessing its byte view past its lifetime will not work
 /// ```
pub fn any_as_u8_slice<'a,T: Sized>(p: &'a T) -> &'a [u8] {
    unsafe{
    ::core::slice::from_raw_parts((p as *const T) as *const u8, ::core::mem::size_of::<T>())
     }
 }
 
 /// Same as any_as_u8_slice but with mutable references. This one is obviously unsafe, as you can
 /// modify any byte of the "transformed" type with this. You have been warned.
pub unsafe fn any_as_u8_slice_mut<'a,T: Sized>(p: &'a mut T) -> &'a mut [u8] {
     {
    ::core::slice::from_raw_parts_mut((p as *mut T) as *mut u8, ::core::mem::size_of::<T>())
     }
 }
 
         type Item = &'a mut i32;
 
         fn next(&mut self) -> Option<Self::Item> {
            let rv = self.data.get_mut(self.index)?;            
             self.index += 1;
            // Here we rebind the lifetime of rv to match the Iterator's own lifetime. 
             // this is perfectly safe, but borrowck is too stupid to know that.
            Some(unsafe{lifetime_detach(rv)})
         }
     }
    
     #[test]
     pub fn detach_mut() {
         let mut data = [0, 1, 2, 3, 4, 5];
         assert_eq!(s, 0);
     }
 }