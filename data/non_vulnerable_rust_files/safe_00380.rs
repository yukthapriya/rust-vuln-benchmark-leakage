// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0087 (FIXED)
// Crate: simd-json-derive
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: simd-lite/simd-json-derive
// Commit: e17eddd
// File: simd-json-derive-int/src/deserialize/struct/named.rs


}
                                 )*
                                 __unknown_field if #deny_unknown_fields => {
                                    return Err(::simd_json_derive::de::Error::UnknownField {
                                        unknown_field: __unknown_field.to_string(),
                                        possible_field_names: &[ #(#value_keys,)* #(#option_keys,)* ]
                                    });
                                 }
                                 _ => {
                                     // ignore unknown field