// REAL FIXED CODE
// RustSec ID: RUSTSEC-2019-0039 (FIXED)
// Crate: typemap
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: reem/rust-typemap
// Commit: 9452d89
// File: src/lib.rs


_assert_debug::<DebugMap>();
 }
 
/// This trait defines the relationship between keys and values in a `TypeMap`.
 ///
 /// It is implemented for Keys, with a phantom associated type for the values.
 pub trait Key: Any {
     }
 
     /// Get the given key's corresponding entry in the map for in-place manipulation.
    pub fn entry<K: Key>(&mut self) -> Entry<K, A>
     where K::Value: Any + Implements<A> {
         match self.data.entry(TypeId::of::<K>()) {
             hash_map::Entry::Occupied(e) => Occupied(OccupiedEntry { data: e, _marker: PhantomData }),
     }
 }
 
/// A view onto an entry in a `TypeMap`.
 pub enum Entry<'a, K, A: ?Sized + UnsafeAnyExt + 'a = UnsafeAny> {
     /// A view onto an occupied entry in a TypeMap.
     Occupied(OccupiedEntry<'a, K, A>),
     /// A view onto an unoccupied entry in a TypeMap.
     Vacant(VacantEntry<'a, K, A>)
 }
 
impl<'a, K: Key, A: ?Sized + UnsafeAnyExt + 'a> Entry<'a, K, A> {
     /// Ensures a value is in the entry by inserting the default if empty, and returns
     /// a mutable reference to the value in the entry.
     pub fn or_insert(self, default: K::Value) -> &'a mut K::Value
     }
 }
 
/// A view onto an occupied entry in a `TypeMap`.
 pub struct OccupiedEntry<'a, K, A: ?Sized + UnsafeAnyExt + 'a = UnsafeAny> {
     data: hash_map::OccupiedEntry<'a, TypeId, Box<A>>,
     _marker: PhantomData<K>
 }
 
/// A view onto an unoccupied entry in a `TypeMap`.
 pub struct VacantEntry<'a, K, A: ?Sized + UnsafeAnyExt + 'a = UnsafeAny> {
     data: hash_map::VacantEntry<'a, TypeId, Box<A>>,
     _marker: PhantomData<K>