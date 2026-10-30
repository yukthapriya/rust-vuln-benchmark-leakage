// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0011 (FIXED)
// Crate: fil-ocl
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: cogciprocate/ocl
// Commit: c5645d1
// File: ocl-core/ocl-core-vector/src/vectors.rs


pub const i16: u32 = (1 << 4) - 1;
     pub const i32: u32 = (1 << 5) - 1;
     pub const i64: u32 = (1 << 6) - 1;
    // pub use self::platform::isize;
 
     pub const u8: u32 = i8;
     pub const u16: u32 = i16;
     pub const u32: u32 = i32;
     pub const u64: u32 = i64;
    // pub use self::platform::usize;
 
     // Char, Char2, Char3, Char4, Char8, Char16,
     // Uchar, Uchar2, Uchar3, Uchar4, Uchar8, Uchar16,