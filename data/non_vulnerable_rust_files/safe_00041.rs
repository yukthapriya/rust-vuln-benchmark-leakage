// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0022 (FIXED)
// Crate: portaudio-rs
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: mvdnes/portaudio-rs
// Commit: 47f1400
// File: src/stream.rs


use pa::{PaError, PaResult};
 use device::DeviceIndex;
 use util::{to_pa_result, pa_time_to_duration, duration_to_pa_time};
 use std::time::Duration;
 use libc::{c_void, c_ulong};
 use std::io::prelude::*;
                                     status_flags: ll::PaStreamCallbackFlags,
                                     user_data: *mut c_void) -> ::libc::c_int
 {
    // We do not want to deallocate this memory since it is owned by other user code. So leak the box.
    let stream_data: &mut StreamUserData<I, O> = Box::leak( unsafe { Box::from_raw(user_data as *mut StreamUserData<I, O>) } );
 
     let input_buffer: &[I] = unsafe
     {
         None => StreamCallbackResult::Abort,
     };
 
     result as i32
 }
 
 extern "C" fn stream_finished_callback<I, O>(user_data: *mut c_void)
 {
    // We do not want to deallocate this memory since it is owned by other user code. So leak the box.
    let stream_data: &mut StreamUserData<I, O> = Box::leak( unsafe { Box::from_raw(user_data as *mut StreamUserData<I, O>) } );
     if let Some(ref mut f) = stream_data.finished_callback
     {
          (*f)();
     };
 }
 
 /// Types that are allowed to be used as samples in a Stream