// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0127 (FIXED)
// Crate: serde_cbor
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: pyfisch/cbor
// Commit: a218403
// File: src/tags.rs


macro_rules! delegate {
     ($name: ident, $type: ty) => {
        fn $name<E: serde::de::Error>(self, v: $type) -> Result<Self::Value, E> {
             T::deserialize(v.into_deserializer()).map(untagged)
         }
     };