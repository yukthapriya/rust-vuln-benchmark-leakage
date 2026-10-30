// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0020
// CVE: N/A
// CWE: N/A
// Crate: const-cstr
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/const-cstr
// Extraction Method: repo_search
// GitHub Repo: abonander/const-cstr
// Commit: 8156b5d
// File: src/lib.rs


//!
 //! ```rust
 //! #[macro_use] extern crate const_cstr;
//! // Just for the `libc::c_char` type alias.
//! extern crate libc;
//!     
 //! use std::ffi::CStr;
 //!
 //! const_cstr! {
 //! }
 //!
 //! // Imagine this is an `extern "C"` function linked from some other lib.
//! unsafe fn print_c_string(cstr: *const libc::c_char) {
 //!     println!("{}", CStr::from_ptr(cstr).to_str().unwrap());
 //! }
 //!
 //! Hello, world!
 //! Goodnight, sun!
 //! ```
extern crate libc;
 
 use std::ffi::CStr;
 
 /// A type representing a static C-compatible string, wrapping `&'static str`.
     /// ------
     /// If the wrapped string is not NUL-terminated. 
     /// (Unlikely if you used the `const_cstr!` macro. This is just a sanity check.)
    pub fn as_ptr(&self) -> *const libc::c_char {
         let bytes = self.val.as_bytes();
 
         assert_eq!(bytes[bytes.len() - 1], b'\0');
 
        self.val.as_bytes().as_ptr() as *const libc::c_char
     }
 
     /// Returns the wrapped string as an `&'static CStr`, skipping the length check that
 /// Remember that functions consuming a C-string will only see up to the first NUL byte.
 #[macro_export]
 macro_rules! const_cstr {
     ($strval:expr) => (
         $crate::ConstCStr { val: concat!($strval, "\0") }
     );
             const $strname: $crate::ConstCStr = const_cstr!($strval);
         )+
     );
    ($(pub $strname:ident = $strval:expr);+;) => (
        $(
            pub const $strname: $crate::ConstCStr = const_cstr!($strval);
        )+
    );
 }
 
 #[test]
     assert_eq!(HELLO_CSTR.to_str(), cstr.to_str().unwrap());
 }