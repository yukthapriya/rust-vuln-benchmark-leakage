// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0375 (FIXED)
// Crate: atty
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: softprops/atty
// Commit: e5e3fc7
// File: src/lib.rs


#[cfg(windows)]
 use winapi::shared::minwindef::DWORD;
#[cfg(windows)]
use winapi::shared::ntdef::WCHAR;
 
 /// possible stream sources
 #[derive(Clone, Copy, Debug)]
 /// Returns true if there is an MSYS tty on the given handle.
 #[cfg(windows)]
 unsafe fn msys_tty_on(fd: DWORD) -> bool {
     use std::mem;
     use std::slice;
 
     use winapi::ctypes::c_void;
     use winapi::shared::minwindef::MAX_PATH;
 
     let size = mem::size_of::<FILE_NAME_INFO>();
    let mut name_info_bytes = vec![0u8; size + MAX_PATH * mem::size_of::<WCHAR>()];
     let res = GetFileInformationByHandleEx(
         GetStdHandle(fd),
         FileNameInfo,
     if res == 0 {
         return false;
     }
    let name_info: &FILE_NAME_INFO = &*(name_info_bytes.as_ptr() as *const FILE_NAME_INFO);
    let s = slice::from_raw_parts(
        name_info.FileName.as_ptr(),
        name_info.FileNameLength as usize / 2,
     );
    let name = String::from_utf16_lossy(s);
     // This checks whether 'pty' exists in the file name, which indicates that
     // a pseudo-terminal is attached. To mitigate against false positives
     // (e.g., an actual file name that contains 'pty'), we also require that