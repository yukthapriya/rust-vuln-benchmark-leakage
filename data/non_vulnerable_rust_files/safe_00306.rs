// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0076 (FIXED)
// Crate: wasmtime
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: bytecodealliance/wasmtime
// Commit: e60c374
// File: crates/runtime/src/instance/allocator/pooling.rs


use std::mem;
 use std::sync::Mutex;
 use wasmtime_environ::{
    DefinedMemoryIndex, DefinedTableIndex, HostPtr, MemoryStyle, Module, PrimaryMap, Tunables,
    VMOffsets, WASM_PAGE_SIZE,
 };
 
 mod index_allocator;
                 .defined_memory_index(memory_index)
                 .expect("should be a defined memory since we skipped imported ones");
 
            match plan.style {
                MemoryStyle::Static { bound } => {
                    let bound = bound * u64::from(WASM_PAGE_SIZE);
                    if bound < self.memories.static_memory_bound {
                        return Err(InstantiationError::Resource(anyhow!(
                            "static bound of {bound:x} bytes incompatible with \
                             reservation of {:x} bytes",
                            self.memories.static_memory_bound,
                        )));
                    }
                }
                MemoryStyle::Dynamic { .. } => {}
            }

             let memory = unsafe {
                 std::slice::from_raw_parts_mut(
                     self.memories.get_base(instance_index, defined_index),
     initial_memory_offset: usize,
     max_memories: usize,
     max_instances: usize,
    static_memory_bound: u64,
 }
 
 impl MemoryPool {
             );
         }
 
        let static_memory_bound =
            u64::from(tunables.static_memory_bound) * u64::from(WASM_PAGE_SIZE);
        let memory_size =
            usize::try_from(static_memory_bound + tunables.static_memory_offset_guard_size)
                .map_err(|_| anyhow!("memory reservation size exceeds addressable memory"))?;
 
         assert!(
             memory_size % crate::page_size() == 0,
             max_memories,
             max_instances,
             max_memory_size: (instance_limits.memory_pages as usize) * (WASM_PAGE_SIZE as usize),
            static_memory_bound,
         };
 
         Ok(pool)