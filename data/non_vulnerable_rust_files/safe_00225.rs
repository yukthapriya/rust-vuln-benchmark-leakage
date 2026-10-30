// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0095 (FIXED)
// Crate: mopa
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: chris-morgan/mopa
// Commit: 2bb823b
// File: src/lib.rs


#[doc(hidden)]
 pub mod __ {
     pub use core::any::TypeId;
     // Option and Result are in the prelude, but they might have been overridden in the macro’s
     // scope, so we do it this way to avoid issues. (Result in particular gets overridden fairly
     // often.)
     fn get_type_id(&self) -> TypeId { TypeId::of::<T>() }
 }
 
 /// The macro for implementing all the `Any` methods on your own trait.
 ///
 /// # Instructions for use
             /// If you are not *absolutely certain* of `T`, you *must not* call this.
             #[inline]
             pub unsafe fn downcast_unchecked<T: $trait_>(self: Box<Self>) -> Box<T> {
                Box::from_raw(Box::into_raw(self) as *mut T)
             }
         }
     };
             /// If you are not *absolutely certain* of `T`, you *must not* call this.
             #[inline]
             pub unsafe fn downcast_ref_unchecked<T: $trait_>(&self) -> &T {
                &*(self as *const Self as *const T)
             }
 
             /// Returns some mutable reference to the boxed value if it is of type `T`, or
             /// If you are not *absolutely certain* of `T`, you *must not* call this.
             #[inline]
             pub unsafe fn downcast_mut_unchecked<T: $trait_>(&mut self) -> &mut T {
                &mut *(self as *mut Self as *mut T)
             }
         }
     };