// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0018 (FIXED)
// Crate: totp-rs
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: constantoine/totp-rs
// Commit: d79bac7
// File: src/lib.rs


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
 #[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
 pub enum Algorithm {
    /// HMAC-SHA1 is the default algorithm of most TOTP implementations.
    /// Some will outright ignore the algorithm parameter to force using SHA1, leading to confusion.
     SHA1,
    /// HMAC-SHA256. Supported in theory according to [yubico](https://docs.yubico.com/yesdk/users-manual/application-oath/uri-string-format.html).
    /// Ignored in practice by most.
     SHA256,
    /// HMAC-SHA512. Supported in theory according to [yubico](https://docs.yubico.com/yesdk/users-manual/application-oath/uri-string-format.html).
    /// Ignored in practice by most.
     SHA512,
     #[cfg(feature = "steam")]
     #[cfg_attr(docsrs, doc(cfg(feature = "steam")))]
    /// Steam TOTP token algorithm.
     Steam,
 }
 
impl Default for Algorithm {
     fn default() -> Self {
         Algorithm::SHA1
     }