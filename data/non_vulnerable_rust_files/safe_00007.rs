// REAL FIXED CODE
// RustSec ID: RUSTSEC-2017-0001 (FIXED)
// Crate: sodiumoxide
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: dnaq/sodiumoxide
// Commit: 3057acb
// File: libsodium-sys/build.rs


use std::{
     env,
     path::{Path, PathBuf},
 };
 
         match &*target {
             "aarch64-apple-ios" => {
                 cflags += " -arch arm64";
                cflags += &format!(" -isysroot {}", sdk_dir_ios);
                cflags += &format!(" -mios-version-min={}", ios_version_min);
                 cflags += " -fembed-bitcode";
                 host_arg = "--host=arm-apple-darwin10".to_string();
             }
             "armv7-apple-ios" => {
                 cflags += " -arch armv7";
                cflags += &format!(" -isysroot {}", sdk_dir_ios);
                cflags += &format!(" -mios-version-min={}", ios_version_min);
                 cflags += " -mthumb";
                 host_arg = "--host=arm-apple-darwin10".to_string();
             }
             "armv7s-apple-ios" => {
                 cflags += " -arch armv7s";
                cflags += &format!(" -isysroot {}", sdk_dir_ios);
                cflags += &format!(" -mios-version-min={}", ios_version_min);
                 cflags += " -mthumb";
                 host_arg = "--host=arm-apple-darwin10".to_string();
             }
             "i386-apple-ios" => {
                 cflags += " -arch i386";
                cflags += &format!(" -isysroot {}", sdk_dir_simulator);
                cflags += &format!(" -mios-simulator-version-min={}", ios_simulator_version_min);
                 host_arg = "--host=i686-apple-darwin10".to_string();
             }
             "x86_64-apple-ios" => {
                 cflags += " -arch x86_64";
                cflags += &format!(" -isysroot {}", sdk_dir_simulator);
                cflags += &format!(" -mios-simulator-version-min={}", ios_simulator_version_min);
                 host_arg = "--host=x86_64-apple-darwin10".to_string();
             }
             _ => panic!("Unknown iOS build target: {}", target),