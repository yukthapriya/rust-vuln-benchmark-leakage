// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0095
// CVE: CVE-2021-45695
// CWE: N/A
// Crate: mopa
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/mopa
// Extraction Method: repo_search
// GitHub Repo: chris-morgan/mopa
// Commit: 2bb823b
// File: src/lib.rs


#[doc(hidden)]
 pub mod __ {
     pub use core::any::TypeId;
    pub use core::mem::transmute;
     // Option and Result are in the prelude, but they might have been overridden in the macro’s
     // scope, so we do it this way to avoid issues. (Result in particular gets overridden fairly
     // often.)
     fn get_type_id(&self) -> TypeId { TypeId::of::<T>() }
 }
 
// Not using core::raw::TraitObject, even if feature = "unstable", because of its feature(raw)
// dependency. It’d be possible to arrange, but it’d require the macro user to add feature(raw).
#[repr(C)]
#[derive(Copy, Clone)]
#[doc(hidden)]
pub struct TraitObject {
    pub data: *mut (),
    pub vtable: *mut (),
}

 /// The macro for implementing all the `Any` methods on your own trait.
 ///
 /// # Instructions for use
             /// If you are not *absolutely certain* of `T`, you *must not* call this.
             #[inline]
             pub unsafe fn downcast_unchecked<T: $trait_>(self: Box<Self>) -> Box<T> {
                let trait_object: $crate::TraitObject = $crate::__::transmute(self);
                $crate::__::transmute(trait_object.data)
             }
         }
     };
             /// If you are not *absolutely certain* of `T`, you *must not* call this.
             #[inline]
             pub unsafe fn downcast_ref_unchecked<T: $trait_>(&self) -> &T {
                let trait_object: $crate::TraitObject = $crate::__::transmute(self);
                $crate::__::transmute(trait_object.data)
             }
 
             /// Returns some mutable reference to the boxed value if it is of type `T`, or
             /// If you are not *absolutely certain* of `T`, you *must not* call this.
             #[inline]
             pub unsafe fn downcast_mut_unchecked<T: $trait_>(&mut self) -> &mut T {
                let trait_object: $crate::TraitObject = $crate::__::transmute(self);
                $crate::__::transmute(trait_object.data)
             }
         }
     };