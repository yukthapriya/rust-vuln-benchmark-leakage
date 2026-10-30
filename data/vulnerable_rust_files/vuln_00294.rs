// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0056
// CVE: N/A
// CWE: N/A
// Crate: clipboard
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/clipboard
// Extraction Method: repo_search
// GitHub Repo: aweinstock314/rust-clipboard
// Commit: 07d080b
// File: src/lib.rs


#![crate_type = "dylib"]
 #![crate_type = "rlib"]
 
#[cfg(all(unix, not(any(target_os="macos", target_os="android"))))]
 extern crate x11_clipboard as x11_clipboard_crate;
 
 #[cfg(windows)]
 mod common;
 pub use common::ClipboardProvider;
 
#[cfg(all(unix, not(any(target_os="macos", target_os="android"))))]
 pub mod x11_clipboard;
 
 #[cfg(windows)]
 
 pub mod nop_clipboard;
 
#[cfg(all(unix, not(any(target_os="macos", target_os="android"))))]
 pub type ClipboardContext = x11_clipboard::X11ClipboardContext;
 #[cfg(windows)]
 pub type ClipboardContext = windows_clipboard::WindowsClipboardContext;
 #[cfg(target_os="macos")]
 pub type ClipboardContext = osx_clipboard::OSXClipboardContext;
 #[cfg(target_os="android")]
 pub type ClipboardContext = nop_clipboard::NopClipboardContext; // TODO: implement AndroidClipboardContext (see #52)
#[cfg(not(any(unix, windows, target_os="macos", target_os="android")))]
 pub type ClipboardContext = nop_clipboard::NopClipboardContext;
 
 #[test]