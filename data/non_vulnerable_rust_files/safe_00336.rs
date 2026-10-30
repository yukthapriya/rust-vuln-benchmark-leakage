// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0020 (FIXED)
// Crate: const-cstr
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: abonander/const-cstr
// Commit: 8156b5d
// File: src/lib.rs


//!
 //! ```rust
 //! #[macro_use] extern crate const_cstr;
//!
//! use std::os::raw::c_char;
 //! use std::ffi::CStr;
 //!
 //! const_cstr! {
 //! }
 //!
 //! // Imagine this is an `extern "C"` function linked from some other lib.
//! unsafe fn print_c_string(cstr: *const c_char) {
 //!     println!("{}", CStr::from_ptr(cstr).to_str().unwrap());
 //! }
 //!
 //! Hello, world!
 //! Goodnight, sun!
 //! ```
 
use std::os::raw::c_char;
 use std::ffi::CStr;
 
 /// A type representing a static C-compatible string, wrapping `&'static str`.
     /// ------
     /// If the wrapped string is not NUL-terminated. 
     /// (Unlikely if you used the `const_cstr!` macro. This is just a sanity check.)
    pub fn as_ptr(&self) -> *const c_char {
         let bytes = self.val.as_bytes();
 
         assert_eq!(bytes[bytes.len() - 1], b'\0');
 
        self.val.as_bytes().as_ptr() as *const c_char
     }
 
     /// Returns the wrapped string as an `&'static CStr`, skipping the length check that
 /// Remember that functions consuming a C-string will only see up to the first NUL byte.
 #[macro_export]
 macro_rules! const_cstr {
    ($(pub $strname:ident = $strval:expr);+;) => (
        $(
            pub const $strname: $crate::ConstCStr = const_cstr!($strval);
        )+
    );
     ($strval:expr) => (
         $crate::ConstCStr { val: concat!($strval, "\0") }
     );
             const $strname: $crate::ConstCStr = const_cstr!($strval);
         )+
     );
 }
 
 #[test]
     assert_eq!(HELLO_CSTR.to_str(), cstr.to_str().unwrap());
 }
 
#[cfg(test)]
mod test_creates_pub_str_mod {
    const_cstr! {
        pub FIRST = "first";
        pub SECOND = "second";
    }
}

#[test]
fn test_creates_pub_str() {
    assert_eq!(test_creates_pub_str_mod::FIRST.to_str(), "first");
    assert_eq!(test_creates_pub_str_mod::SECOND.to_str(), "second");
}