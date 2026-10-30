// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0085
// CVE: N/A
// CWE: N/A
// Crate: hpack
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/hpack
// Extraction Method: commit_diff
// GitHub Repo: sno2/hpack-rs-patched
// Commit: d669282
// File: src/decoder.rs


},
                 FieldRepresentation::SizeUpdate => {
                     // Handle the dynamic table size update...
                    self.update_max_dynamic_size(buffer_leftover)
                 }
             };
 
     /// size of the underlying dynamic table, possibly causing a number of
     /// headers to be evicted from it.
     ///
    /// Assumes that the first byte in the given buffer `buf` is the first
    /// octet in the `SizeUpdate` block.
    ///
     /// Returns the number of octets consumed from the given buffer.
    fn update_max_dynamic_size(&mut self, buf: &[u8]) -> usize {
        let (new_size, consumed) = decode_integer(buf, 5).ok().unwrap();
         self.header_table.dynamic_table.set_max_table_size(new_size);
 
         info!("Decoder changed max table size from {} to {}",
               self.header_table.dynamic_table.get_size(),
               new_size);
 
        consumed
     }
 }
 
             _ => false,
         });
     }
 }
 
 /// The module defines interop tests between this HPACK decoder