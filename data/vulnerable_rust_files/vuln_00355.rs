// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2023-0048
// CVE: N/A
// CWE: N/A
// Crate: intaglio
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/intaglio
// Extraction Method: repo_search
// GitHub Repo: artichoke/intaglio
// Commit: c014fc9
// File: src/bytes.rs


use core::ops::Range;
 use core::slice;
 use std::borrow::Cow;
use std::collections::hash_map::{HashMap, RandomState};
 
 use crate::internal::Interned;
 use crate::{Symbol, SymbolOverflowError, DEFAULT_SYMBOL_TABLE_CAPACITY};
         self.vec.reserve(additional);
     }
 
     /// Shrinks the capacity of the symbol table as much as possible.
     ///
     /// It will drop down as close as possible to the length but the allocator
         assert_eq!(sym, table.intern(&b"abc"[..]).unwrap());
     }
 
     quickcheck! {
         fn intern_twice_symbol_equality(bytes: Vec<u8>) -> bool {
             let mut table = SymbolTable::new();