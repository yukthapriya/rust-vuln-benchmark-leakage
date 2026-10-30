// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0075
// CVE: CVE-2022-39393
// CWE: N/A
// Crate: wasmtime
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/wasmtime
// Extraction Method: commit_diff
// GitHub Repo: bytecodealliance/wasmtime
// Commit: 2614f2e
// File: crates/runtime/src/instance/allocator/pooling.rs


)
             };
 
             if let Some(image) = runtime_info
                 .memory_image(defined_index)
                 .map_err(|err| InstantiationError::Resource(err.into()))?
             {
                let mut slot = self
                    .memories
                    .take_memory_image_slot(instance_index, defined_index);
                 let initial_size = plan.memory.minimum * WASM_PAGE_SIZE as u64;
 
                 // If instantiation fails, we can propagate the error
                     .map_err(InstantiationError::Resource)?,
                 );
             } else {
                 memories.push(
                     Memory::new_static(plan, memory, Some(commit_memory_pages), None, unsafe {
                         &mut *store.unwrap()