// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0005 (FIXED)
// Crate: cbox
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: TomBebbington/cbox-rs
// Commit: 2fc8735
// File: src/lib.rs


unsafe { mem::transmute(self.ptr) }
     }
 }
impl<'a, T> PartialEq<T> for CBox<T> where T:'a+DisposeRef+PartialEq, *mut T::RefTo:Into<&'a T> {
     fn eq(&self, other: &T) -> bool {
         unsafe {
             mem::transmute::<_, &T>(self.ptr) == other