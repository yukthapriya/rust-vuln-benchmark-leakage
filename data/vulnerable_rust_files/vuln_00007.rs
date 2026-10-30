// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2017-0001
// CVE: CVE-2017-1000168
// CWE: N/A
// Crate: sodiumoxide
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/sodiumoxide
// Extraction Method: repo_search
// GitHub Repo: dnaq/sodiumoxide
// Commit: 3057acb
// File: libsodium-sys/build.rs


use std::{
     env,
    fmt::Write,
     path::{Path, PathBuf},
 };
 
         match &*target {
             "aarch64-apple-ios" => {
                 cflags += " -arch arm64";
                write!(cflags, " -isysroot {}", sdk_dir_ios).unwrap();
                write!(cflags, " -mios-version-min={}", ios_version_min).unwrap();
                 cflags += " -fembed-bitcode";
                 host_arg = "--host=arm-apple-darwin10".to_string();
             }
             "armv7-apple-ios" => {
                 cflags += " -arch armv7";
                write!(cflags, " -isysroot {}", sdk_dir_ios).unwrap();
                write!(cflags, " -mios-version-min={}", ios_version_min).unwrap();
                 cflags += " -mthumb";
                 host_arg = "--host=arm-apple-darwin10".to_string();
             }
             "armv7s-apple-ios" => {
                 cflags += " -arch armv7s";
                write!(cflags, " -isysroot {}", sdk_dir_ios).unwrap();
                write!(cflags, " -mios-version-min={}", ios_version_min).unwrap();
                 cflags += " -mthumb";
                 host_arg = "--host=arm-apple-darwin10".to_string();
             }
             "i386-apple-ios" => {
                 cflags += " -arch i386";
                write!(cflags, " -isysroot {}", sdk_dir_simulator).unwrap();
                write!(
                    cflags,
                    " -mios-simulator-version-min={}",
                    ios_simulator_version_min
                )
                .unwrap();
                 host_arg = "--host=i686-apple-darwin10".to_string();
             }
             "x86_64-apple-ios" => {
                 cflags += " -arch x86_64";
                write!(cflags, " -isysroot {}", sdk_dir_simulator).unwrap();
                write!(
                    cflags,
                    " -mios-simulator-version-min={}",
                    ios_simulator_version_min
                )
                .unwrap();
                 host_arg = "--host=x86_64-apple-darwin10".to_string();
             }
             _ => panic!("Unknown iOS build target: {}", target),