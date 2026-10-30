// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0443
// CVE: N/A
// CWE: N/A
// Crate: webp
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/webp
// Extraction Method: repo_search
// GitHub Repo: jaredforth/webp
// Commit: 62b4706
// File: src/encoder.rs


use libwebp_sys::*;
 
 use crate::shared::*;
 
 /// An encoder for WebP images. It uses the default configuration of libwebp.
 pub struct Encoder<'a> {
    image: &'a [u8],
    layout: PixelLayout,
    width: u32,
    height: u32,
 }
 
 impl<'a> Encoder<'a> {
     /// Creates a new encoder from the given image data.
     /// The image data must be in the pixel layout of the color parameter.
     pub fn new(image: &'a [u8], layout: PixelLayout, width: u32, height: u32) -> Self {
         Self {
            image,
            layout,
            width,
            height,
         }
     }
 
     /// Creates a new encoder from the given image data in the RGB pixel layout.
     pub fn from_rgb(image: &'a [u8], width: u32, height: u32) -> Self {
         Self {
            image,
            layout: PixelLayout::Rgb,
            width,
            height,
         }
     }
 
     /// Creates a new encoder from the given image data in the RGBA pixel layout.
     pub fn from_rgba(image: &'a [u8], width: u32, height: u32) -> Self {
         Self {
            image,
            layout: PixelLayout::Rgba,
            width,
            height,
         }
     }
 
 
     pub fn encode_advanced(&self, config: &WebPConfig) -> Result<WebPMemory, WebPEncodingError> {
         unsafe {
            let mut picture = new_picture(self.image, self.layout, self.width, self.height);
            encode(&mut picture, config)
         }
     }
 }
     use super::*;
     use crate::shared;
 
     #[test]
     fn test_encoder_new_assigns_fields() {
        let data = [1, 2, 3, 4, 5, 6];
         let enc = Encoder::new(&data, shared::PixelLayout::Rgb, 2, 3);
        assert_eq!(enc.image, &data);
        assert_eq!(enc.layout, shared::PixelLayout::Rgb);
        assert_eq!(enc.width, 2);
        assert_eq!(enc.height, 3);
     }
 
     #[test]
         let rgba = [1, 2, 3, 4, 5, 6, 7, 8];
         let enc_rgb = Encoder::from_rgb(&rgb, 2, 1);
         let enc_rgba = Encoder::from_rgba(&rgba, 2, 1);
        assert_eq!(enc_rgb.layout, shared::PixelLayout::Rgb);
        assert_eq!(enc_rgba.layout, shared::PixelLayout::Rgba);
        assert_eq!(enc_rgb.image, &rgb);
        assert_eq!(enc_rgba.image, &rgba);
        assert_eq!(enc_rgb.width, 2);
        assert_eq!(enc_rgba.height, 1);
     }
 
     #[cfg(feature = "img")]