// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0087
// CVE: N/A
// CWE: N/A
// Crate: simd-json-derive
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/simd-json-derive
// Extraction Method: repo_search
// GitHub Repo: simd-lite/simd-json-derive
// Commit: e17eddd
// File: simd-json-derive-int/src/deserialize/struct/named.rs


}
                                 )*
                                 __unknown_field if #deny_unknown_fields => {
                                    return Err(::simd_json_derive::de::Error::UnknownField(__unknown_field.to_string(), &[ #(#value_keys,)* #(#option_keys,)* ]));
                                 }
                                 _ => {
                                     // ignore unknown field