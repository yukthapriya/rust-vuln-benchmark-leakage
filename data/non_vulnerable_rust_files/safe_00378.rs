// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0085 (FIXED)
// Crate: hpack
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: sno2/hpack-rs-patched
// Commit: d669282
// File: src/decoder.rs


},
                 FieldRepresentation::SizeUpdate => {
                     // Handle the dynamic table size update...
                    self.update_max_dynamic_size(buffer_leftover)?
                 }
             };
 
     /// size of the underlying dynamic table, possibly causing a number of
     /// headers to be evicted from it.
     ///
     /// Returns the number of octets consumed from the given buffer.
    fn update_max_dynamic_size(&mut self, buf: &[u8]) -> Result<usize, DecoderError> {
        let (new_size, consumed) = decode_integer(buf, 5)?;
         self.header_table.dynamic_table.set_max_table_size(new_size);
 
         info!("Decoder changed max table size from {} to {}",
               self.header_table.dynamic_table.get_size(),
               new_size);
 
        Ok(consumed)
     }
 }
 
             _ => false,
         });
     }

    #[test]
    fn test_invalid_dynamic_size_update() {
        let mut decoder = Decoder::new();
        let hex_dump = &[0x3f];
        let _ = decoder.decode(hex_dump);
    }
 }
 
 /// The module defines interop tests between this HPACK decoder