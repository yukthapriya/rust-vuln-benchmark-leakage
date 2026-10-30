// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0145 (FIXED)
// Crate: heapless
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: japaric/heapless
// Commit: a3fc143
// File: src/pool/arc.rs


use core::{
     fmt,
     hash::{Hash, Hasher},
    mem::MaybeUninit,
     ops, ptr,
 };
 
     fn manage(&self, block: &'static mut ArcBlock<T>) {
         let node: &'static mut _ = &mut block.node;
 
        // SAFETY: The node within an `ArcBlock` is always properly initialized for linking because
        // the only way for         client code to construct an `ArcBlock` is through
        // `ArcBlock::new`. The `NonNullPtr` comes from a         reference, so it is
        // guaranteed to be dereferencable. It is also unique because the `ArcBlock` itself
        //         is passed as a `&mut`
         unsafe { self.stack.push(NonNullPtr::from_static_mut_ref(node)) }
     }
 }
     /// Creates a new memory block
     pub const fn new() -> Self {
         Self {
            node: UnionNode::unlinked(),
         }
     }
 }