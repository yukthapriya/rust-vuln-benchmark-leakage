// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0443 (FIXED)
// Crate: webp
// Type: memory_safety (FIXED)
// Extraction Method: repo_search
// GitHub Repo: jaredforth/webp
// Commit: 62b4706
// File: src/encoder.rs


use libwebp_sys::*;
 
 use crate::shared::*;
use internal::CheckedEncoder;
 
 /// An encoder for WebP images. It uses the default configuration of libwebp.
 pub struct Encoder<'a> {
    e: CheckedEncoder<'a>,
 }
 
 impl<'a> Encoder<'a> {
     /// Creates a new encoder from the given image data.
     /// The image data must be in the pixel layout of the color parameter.
     pub fn new(image: &'a [u8], layout: PixelLayout, width: u32, height: u32) -> Self {
         Self {
            e: CheckedEncoder::new(image, layout, width, height),
         }
     }
 
     /// Creates a new encoder from the given image data in the RGB pixel layout.
     pub fn from_rgb(image: &'a [u8], width: u32, height: u32) -> Self {
         Self {
            e: CheckedEncoder::new(image, PixelLayout::Rgb, width, height),
         }
     }
 
     /// Creates a new encoder from the given image data in the RGBA pixel layout.
     pub fn from_rgba(image: &'a [u8], width: u32, height: u32) -> Self {
         Self {
            e: CheckedEncoder::new(image, PixelLayout::Rgba, width, height),
         }
     }
 
 
     pub fn encode_advanced(&self, config: &WebPConfig) -> Result<WebPMemory, WebPEncodingError> {
         unsafe {
            let mut picture = new_picture(
                self.e.image(),
                self.e.layout(),
                self.e.width(),
                self.e.height(),
            );
            let res = encode(&mut *picture, config);
            res
        }
    }
}

/// This module contains the private CheckedEncoder so that it cannot be constructed directly from the outside.
/// That ensures that the only way to construct one validates the supplied parameters.
mod internal {
    use std::convert::TryInto;

    use crate::shared::PixelLayout;

    /// Validated encoder parameters, guaranteeing `image.len() >= width * height * bytes_per_pixel`.
    /// Required for memory safety. Their absence would allow out-of-bounds reads.
    pub(super) struct CheckedEncoder<'a> {
        image: &'a [u8],
        layout: PixelLayout,
        width: u32,
        height: u32,
    }

    impl<'a> CheckedEncoder<'a> {
        /// Creates a new instance of `CheckedEncoder`, and performs the necessary bounds checks.
        ///
        /// This is the only way to create a `CheckedEncoder` exposed outside the `internal` module.
        pub(super) fn new(image: &'a [u8], layout: PixelLayout, width: u32, height: u32) -> Self {
            // TODO: return an error instead of panicking in the next semver-breaking release

            if width == 0 || height == 0 {
                // TODO: enable this check once this function returns a Result.
                // As of v0.3.0 it's possible to contruct an Encoder with width or height 0,
                // but encoding will fail and the error is really uninformative:
                // "called `Result::unwrap()` on an `Err` value: VP8_ENC_ERROR_BAD_DIMENSION"

                //panic!("Width and height must be non-zero.");
            }

            // We're going to compare the incoming width * height * bpp against the length of a slice.
            // The length of a slice is a `usize`, so we are going to do arithmetic in `usize` as well.
            //
            // On 32-bit and 64-bit platforms these conversions always suceeed and get optimized out.
            let width_u: usize = width.try_into().unwrap();
            let height_u: usize = height.try_into().unwrap();
            let bytes_per_pixel_u: usize = layout.bytes_per_pixel().into();

            // If we simply calculate `width * height * bytes_per_pixel`, arithmetic may overflow in release mode
            // and make it possible to bypass the length check. So we need either .checked_mul or .saturating_mul here.
            //
            // Using `saturating_mul` is enough because it's not possible to construct a valid slice of size `usize::MAX` anyway,
            // and it makes for simpler code and a nicer error message.
            let expected_len = width_u
                .saturating_mul(height_u)
                .saturating_mul(bytes_per_pixel_u);
            if image.len() < expected_len {
                panic!(
                    "Image buffer too small. Expected at least {} bytes for a {}x{} image with {:?} layout, got {}.",
                    expected_len, width, height, layout, image.len()
                );
            }

            if image.len() > expected_len {
                // TODO: reject this in the future, once this function is made to return Result
            }

            CheckedEncoder {
                image,
                layout,
                width,
                height,
            }
        }

        pub(super) fn width(&self) -> u32 {
            self.width
        }

        pub(super) fn height(&self) -> u32 {
            self.height
        }

        pub(super) fn layout(&self) -> PixelLayout {
            self.layout
        }

        pub(super) fn image(&self) -> &'a [u8] {
            self.image
         }
     }
 }
     use super::*;
     use crate::shared;
 
    #[test]
    #[should_panic]
    fn construct_encoder_with_buffer_overflow() {
        Encoder::from_rgb(&[], 16383, 16383);
    }

     #[test]
     fn test_encoder_new_assigns_fields() {
        let data = [5; 18];
         let enc = Encoder::new(&data, shared::PixelLayout::Rgb, 2, 3);
        assert_eq!(enc.e.image(), &data);
        assert_eq!(enc.e.layout(), shared::PixelLayout::Rgb);
        assert_eq!(enc.e.width(), 2);
        assert_eq!(enc.e.height(), 3);
     }
 
     #[test]
         let rgba = [1, 2, 3, 4, 5, 6, 7, 8];
         let enc_rgb = Encoder::from_rgb(&rgb, 2, 1);
         let enc_rgba = Encoder::from_rgba(&rgba, 2, 1);
        assert_eq!(enc_rgb.e.layout(), shared::PixelLayout::Rgb);
        assert_eq!(enc_rgba.e.layout(), shared::PixelLayout::Rgba);
        assert_eq!(enc_rgb.e.image(), &rgb);
        assert_eq!(enc_rgba.e.image(), &rgba);
        assert_eq!(enc_rgb.e.width(), 2);
        assert_eq!(enc_rgba.e.height(), 1);
     }
 
     #[cfg(feature = "img")]