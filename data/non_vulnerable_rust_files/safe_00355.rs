// REAL FIXED CODE
// RustSec ID: RUSTSEC-2023-0048 (FIXED)
// Crate: intaglio
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: artichoke/intaglio
// Commit: c014fc9
// File: src/bytes.rs


use core::ops::Range;
 use core::slice;
 use std::borrow::Cow;
use std::collections::{
    hash_map::{HashMap, RandomState},
    TryReserveError,
};
 
 use crate::internal::Interned;
 use crate::{Symbol, SymbolOverflowError, DEFAULT_SYMBOL_TABLE_CAPACITY};
         self.vec.reserve(additional);
     }
 
    /// Tries to reserve capacity for at least `additional` more elements in
    /// the `SymbolTable`, returning an error if the allocation fails.
    ///
    /// After calling `try_reserve`, capacity will be greater than or equal to
    /// `self.len() + additional` on success. Does nothing if capacity is
    /// already sufficient.
    ///
    /// # Errors
    ///
    /// Returns a [`TryReserveError`] if the allocation fails or if the new
    /// capacity would overflow `usize`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use intaglio::bytes::SymbolTable;
    /// # fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let mut table = SymbolTable::with_capacity(1);
    /// table.intern(b"abc")?;
    /// table.try_reserve(10)?;
    /// assert!(table.capacity() >= 11);
    /// # Ok(())
    /// # }
    /// # example().unwrap();
    /// ```
    pub fn try_reserve(&mut self, additional: usize) -> Result<(), TryReserveError> {
        self.map.try_reserve(additional)?;
        self.vec.try_reserve(additional)
    }

     /// Shrinks the capacity of the symbol table as much as possible.
     ///
     /// It will drop down as close as possible to the length but the allocator
         assert_eq!(sym, table.intern(&b"abc"[..]).unwrap());
     }
 
    #[test]
    fn try_reserve_grows_capacity() {
        let mut table = SymbolTable::with_capacity(1);
        table.intern(&b"abc"[..]).unwrap();
        let len = table.len();

        table.try_reserve(10).unwrap();

        assert!(table.capacity() >= len + 10);
    }

    #[test]
    fn try_reserve_overflow_errors() {
        let mut table = SymbolTable::new();

        let result = table.try_reserve(usize::MAX);

        assert!(result.is_err());
    }

     quickcheck! {
         fn intern_twice_symbol_equality(bytes: Vec<u8>) -> bool {
             let mut table = SymbolTable::new();