// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2018-0004
// CVE: CVE-2018-20992
// CWE: N/A
// Crate: claxon
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/claxon
// Extraction Method: commit_diff
// GitHub Repo: ruuda/claxon
// Commit: 8f28ec2
// File: src/subframe.rs


// most 2^16 - 1 samples in the block. No values have been marked as
     // invalid by the specification though.
     let n_partitions = 1u32 << order;
    let n_samples = block_size >> order;
     let n_warm_up = block_size - buffer.len() as u16;
 
     // The partition size must be at least as big as the number of warm-up
     // samples, otherwise the size of the first partition is negative.
    if n_warm_up > n_samples {
         return fmt_err("invalid residual");
     }
 
     // Finally decode the partitions themselves.
     match partition_type {
         RicePartitionType::Rice => {
             let mut start = 0;
            let mut len = n_samples - n_warm_up;
             for _ in 0..n_partitions {
                 let slice = &mut buffer[start..start + len as usize];
                 try!(decode_rice_partition(input, slice));
                 start = start + len as usize;
                len = n_samples;
             }
         }
         RicePartitionType::Rice2 => {
             let mut start = 0;
            let mut len = n_samples - n_warm_up;
             for _ in 0..n_partitions {
                 let slice = &mut buffer[start..start + len as usize];
                 try!(decode_rice2_partition(input, slice));
                 start = start + len as usize;
                len = n_samples;
             }
         }
     }