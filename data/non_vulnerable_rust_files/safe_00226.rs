// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0096 (FIXED)
// Crate: spirv_headers
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: gfx-rs/rspirv
// Commit: 8879923
// File: autogen/src/binary.rs


// For each operand kind in the BitEnum category, its
             // enumerants are bit masks. If a certain bit having associated
             // parameters is set, we also need to decode the corresponding
            // parameters. E.g., for MemoryAccess Aligned, an additional
             // LiteralInteger, which stands for the known alignment, should
             // be decoded.
 
                     spirv::#kind::#symbol => vec![#(#params),*]
                 }
             });
             quote! {
                 fn #function_name(&mut self, #lo_kind: spirv::#kind) -> Result<Vec<dr::Operand>> {
                     Ok(match #lo_kind {
                         #(#cases),*,