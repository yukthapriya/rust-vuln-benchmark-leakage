// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0382
// CVE: N/A
// CWE: N/A
// Crate: hwloc
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/hwloc
// Extraction Method: repo_search
// GitHub Repo: daschl/hwloc-rs
// Commit: 3bafbd9
// File: src/bitmap.rs


pub fn new() -> Bitmap {
         let bitmap = unsafe { ffi::hwloc_bitmap_alloc() };
         Bitmap {
            bitmap: bitmap,
             manage: true,
         }
     }
     pub fn full() -> Bitmap {
         let bitmap = unsafe { ffi::hwloc_bitmap_alloc_full() };
         Bitmap {
            bitmap: bitmap,
             manage: true,
         }
     }
     /// conversion factory when dealing with hwloc-internal structures.
     pub fn from_raw(bitmap: *mut IntHwlocBitmap, manage: bool) -> Bitmap {
         Bitmap {
            bitmap: bitmap,
            manage: manage,
         }
     }
 
     /// ```
     pub fn is_empty(&self) -> bool {
         let result = unsafe { ffi::hwloc_bitmap_iszero(self.bitmap) };
        !(result == 0)
     }
 
     /// Check if the field with the given id is set.
     /// ```
     pub fn is_set(&self, id: u32) -> bool {
         let result = unsafe { ffi::hwloc_bitmap_isset(self.bitmap, id) };
        !(result == 0)
     }
 
     /// Keep a single index among those set in the bitmap.
         let mut result: *mut c_char = ptr::null_mut();
         unsafe {
             ffi::hwloc_bitmap_list_asprintf(&mut result, self.bitmap);
            write!(f, "{}", CStr::from_ptr(result).to_str().unwrap())
         }
     }
 }
         let mut result: *mut c_char = ptr::null_mut();
         unsafe {
             ffi::hwloc_bitmap_list_asprintf(&mut result, self.bitmap);
            write!(f, "{}", CStr::from_ptr(result).to_str().unwrap())
         }
     }
 }