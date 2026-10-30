// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0145
// CVE: CVE-2020-36464
// CWE: N/A
// Crate: heapless
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/heapless
// Extraction Method: repo_search
// GitHub Repo: japaric/heapless
// Commit: a3fc143
// File: src/pool/arc.rs


use core::{
     fmt,
     hash::{Hash, Hasher},
    mem::{ManuallyDrop, MaybeUninit},
     ops, ptr,
 };
 
     fn manage(&self, block: &'static mut ArcBlock<T>) {
         let node: &'static mut _ = &mut block.node;
 
         unsafe { self.stack.push(NonNullPtr::from_static_mut_ref(node)) }
     }
 }
     /// Creates a new memory block
     pub const fn new() -> Self {
         Self {
            node: UnionNode {
                data: ManuallyDrop::new(MaybeUninit::uninit()),
            },
         }
     }
 }