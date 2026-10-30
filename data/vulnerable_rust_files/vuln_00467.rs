// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0431
// CVE: N/A
// CWE: N/A
// Crate: xous
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/xous
// Extraction Method: repo_search
// GitHub Repo: betrusted-io/xous-core
// Commit: d531330
// File: baremetal/src/platform/bao1x/bao1x.rs


bao1x_hal::board::setup_camera_pins(&iox);
         bao1x_hal::board::setup_kb_pins(&iox);
         bao1x_hal::board::setup_oled_power_pin(&iox);
        bao1x_hal::board::setup_trng_power_pin(&iox);
         // make sure SE0 is cleared
         let (port, pin) = bao1x_hal::board::setup_usb_pins(&iox);
         iox.set_gpio_pin(port, pin, IoxValue::High);