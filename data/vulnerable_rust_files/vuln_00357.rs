// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0051
// CVE: N/A
// CWE: N/A
// Crate: dlopen_derive
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/dlopen_derive
// Extraction Method: repo_search
// GitHub Repo: szymonwieloch/rust-dlopen
// Commit: f9fed71
// File: src/raw/windows.rs


} else {None};
     Ok({
         AddressInfo{
            dll_path: OsString::from_wide(&buffer[0..(path_len as usize)]).to_string_lossy().into_owned(),
             dll_base_addr: module_base as * const (),
             overlapping_symbol: os,
         }