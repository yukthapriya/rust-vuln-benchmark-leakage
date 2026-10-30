// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0038 (FIXED)
// Crate: libpulse-binding
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: jnqnfe/pulse-binding-rust
// Commit: 7fd282a
// File: pulse-binding/src/context/ext_device_manager.rs


//! Routines for controlling module-device-manager.
 
use std;
 use capi;
 use std::ffi::{CStr, CString};
 use std::borrow::Cow;
 fn read_list_cb_proxy(_: *mut ContextInternal, i: *const InfoInternal, eol: i32,
     userdata: *mut c_void)
 {
    let _ = std::panic::catch_unwind(|| {
        match callback_for_list_instance::<dyn FnMut(ListResult<&Info>)>(eol, userdata) {
            ListInstanceCallback::Entry(callback) => {
                assert!(!i.is_null());
                let obj = Info::new_from_raw(i);
                (callback)(ListResult::Item(&obj));
            },
            ListInstanceCallback::End(mut callback) => { (callback)(ListResult::End); },
            ListInstanceCallback::Error(mut callback) => { (callback)(ListResult::Error); },
        }
    });
 }