// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2021-0145
// CVE: N/A
// CWE: N/A
// Crate: atty
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/atty
// Extraction Method: repo_search
// GitHub Repo: softprops/atty
// Commit: e5e3fc7
// File: src/lib.rs


#[cfg(windows)]
 use winapi::shared::minwindef::DWORD;
 
 /// possible stream sources
 #[derive(Clone, Copy, Debug)]
 /// Returns true if there is an MSYS tty on the given handle.
 #[cfg(windows)]
 unsafe fn msys_tty_on(fd: DWORD) -> bool {
    use std::ffi::OsString;
     use std::mem;
    use std::os::windows::ffi::OsStringExt;
     use std::slice;
 
     use winapi::ctypes::c_void;
     use winapi::shared::minwindef::MAX_PATH;
 
     let size = mem::size_of::<FILE_NAME_INFO>();
    let mut name_info_bytes = vec![0u8; size + MAX_PATH];
     let res = GetFileInformationByHandleEx(
         GetStdHandle(fd),
         FileNameInfo,
     if res == 0 {
         return false;
     }
    let name_info: FILE_NAME_INFO = *(name_info_bytes[0..size].as_ptr() as
                                          *const FILE_NAME_INFO);
    let name_bytes =
        &name_info_bytes[size..size + name_info.FileNameLength as usize];
    let name_u16 = slice::from_raw_parts(
        name_bytes.as_ptr() as *const u16,
        name_bytes.len() / 2,
     );
    let name = OsString::from_wide(name_u16)
        .as_os_str()
        .to_string_lossy()
        .into_owned();
     // This checks whether 'pty' exists in the file name, which indicates that
     // a pseudo-terminal is attached. To mitigate against false positives
     // (e.g., an actual file name that contains 'pty'), we also require that