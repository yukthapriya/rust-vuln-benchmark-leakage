// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0076
// CVE: CVE-2022-39392
// CWE: N/A
// Crate: wasmtime
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/wasmtime
// Extraction Method: commit_diff
// GitHub Repo: bytecodealliance/wasmtime
// Commit: e60c374
// File: crates/runtime/src/instance/allocator/pooling.rs


use std::mem;
 use std::sync::Mutex;
 use wasmtime_environ::{
    DefinedMemoryIndex, DefinedTableIndex, HostPtr, Module, PrimaryMap, Tunables, VMOffsets,
    WASM_PAGE_SIZE,
 };
 
 mod index_allocator;
                 .defined_memory_index(memory_index)
                 .expect("should be a defined memory since we skipped imported ones");
 
             let memory = unsafe {
                 std::slice::from_raw_parts_mut(
                     self.memories.get_base(instance_index, defined_index),
     initial_memory_offset: usize,
     max_memories: usize,
     max_instances: usize,
 }
 
 impl MemoryPool {
             );
         }
 
        let memory_size = if instance_limits.memory_pages > 0 {
            usize::try_from(
                u64::from(tunables.static_memory_bound) * u64::from(WASM_PAGE_SIZE)
                    + tunables.static_memory_offset_guard_size,
            )
            .map_err(|_| anyhow!("memory reservation size exceeds addressable memory"))?
        } else {
            0
        };
 
         assert!(
             memory_size % crate::page_size() == 0,
             max_memories,
             max_instances,
             max_memory_size: (instance_limits.memory_pages as usize) * (WASM_PAGE_SIZE as usize),
         };
 
         Ok(pool)