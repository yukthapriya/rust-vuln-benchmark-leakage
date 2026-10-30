// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0098
// CVE: CVE-2020-36206
// CWE: N/A
// Crate: rusb
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/rusb
// Extraction Method: repo_search
// GitHub Repo: a1ien/rusb
// Commit: cd95bb7
// File: libusb1-sys/build.rs


Some("__attribute__((visibility(\"default\")))"),
         );
 
        if let Ok(lib) = pkg_config::probe_library("libudev") {
            base_config.define("USE_UDEV", Some("1"));
            base_config.define("HAVE_LIBUDEV", Some("1"));
            base_config.file(libusb_source.join("libusb/os/linux_udev.c"));
            for path in lib.include_paths {
                base_config.include(path.to_str().unwrap());
            }
        };
 
         println!("Including posix!");
         base_config.file(libusb_source.join("libusb/os/events_posix.c"));