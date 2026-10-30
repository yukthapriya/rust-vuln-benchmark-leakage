// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0025
// CVE: CVE-2019-25001
// CWE: N/A
// Crate: serde_cbor
// Type: overflow
// Severity: medium
// Source: https://crates.io/crates/serde_cbor
// Extraction Method: repo_search
// GitHub Repo: pyfisch/cbor
// Commit: a218403
// File: src/tags.rs


macro_rules! delegate {
     ($name: ident, $type: ty) => {
        fn $name<E: serde::de::Error>(self, v: $type) -> Result<Self::Value, E>
        {
             T::deserialize(v.into_deserializer()).map(untagged)
         }
     };