// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0065
// CVE: CVE-2021-38187
// CWE: N/A
// Crate: anymap
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/anymap
// Extraction Method: repo_search
// GitHub Repo: chris-morgan/anymap
// Commit: 9351191
// File: src/any.rs


use core::fmt;
use core::any::Any;
 #[cfg(not(feature = "std"))]
 use alloc::boxed::Box;
 
     }
 }
 
#[allow(missing_docs)]  // Bogus warning (it’s not public outside the crate), ☹
pub trait UncheckedAnyExt: Any {
    unsafe fn downcast_ref_unchecked<T: Any>(&self) -> &T;
    unsafe fn downcast_mut_unchecked<T: Any>(&mut self) -> &mut T;
    unsafe fn downcast_unchecked<T: Any>(self: Box<Self>) -> Box<T>;
 }
 
#[doc(hidden)]
 /// A trait for the conversion of an object into a boxed trait object.
pub trait IntoBox<A: ?Sized + UncheckedAnyExt>: Any {
     /// Convert self into the appropriate boxed form.
     fn into_box(self) -> Box<A>;
 }
 
 macro_rules! implement {
    ($base:ident, $(+ $bounds:ident)*) => {
        impl UncheckedAnyExt for dyn $base $(+ $bounds)* {
             #[inline]
             unsafe fn downcast_ref_unchecked<T: 'static>(&self) -> &T {
                 &*(self as *const Self as *const T)
             }
         }
 
        impl<T: $base $(+ $bounds)*> IntoBox<dyn $base $(+ $bounds)*> for T {
             #[inline]
            fn into_box(self) -> Box<dyn $base $(+ $bounds)*> {
                 Box::new(self)
             }
         }
     }
 }
 
implement!(Any,);
implement!(Any, + Send);
implement!(Any, + Send + Sync);
 
 /// [`Any`], but with cloning.
 ///
 /// Every type with no non-`'static` references that implements `Clone` implements `CloneAny`.
 /// See [`core::any`] for more details on `Any` in general.
 pub trait CloneAny: Any + CloneToAny { }
 impl<T: Any + Clone> CloneAny for T { }
implement!(CloneAny,);
implement!(CloneAny, + Send);
implement!(CloneAny, + Send + Sync);
 impl_clone!(dyn CloneAny);
 impl_clone!(dyn CloneAny + Send);
 impl_clone!(dyn CloneAny + Send + Sync);