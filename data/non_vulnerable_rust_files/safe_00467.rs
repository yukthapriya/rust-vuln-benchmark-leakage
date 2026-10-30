// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0431 (FIXED)
// Crate: xous
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: betrusted-io/xous-core
// Commit: d531330
// File: baremetal/src/platform/bao1x/bao1x.rs


bao1x_hal::board::setup_camera_pins(&iox);
         bao1x_hal::board::setup_kb_pins(&iox);
         bao1x_hal::board::setup_oled_power_pin(&iox);

        let trng_power = bao1x_hal::board::setup_trng_power_pin(&iox);
        // kernel expects the TRNG to be on
        iox.set_gpio_pin(trng_power.0, trng_power.1, bao1x_api::IoxValue::High);

        let (port, pin) = bao1x_hal::board::setup_dcdc2_pin(&iox);
        // low connects DCDC2 to the chip
        iox.set_gpio_pin(port, pin, IoxValue::Low);

         // make sure SE0 is cleared
         let (port, pin) = bao1x_hal::board::setup_usb_pins(&iox);
         iox.set_gpio_pin(port, pin, IoxValue::High);