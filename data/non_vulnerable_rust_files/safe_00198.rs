// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0052 (FIXED)
// Crate: id-map
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: andrewhickman/id-map
// Commit: fab6922
// File: src/lib.rs


//! [`IdMap`] is a container that gives each item a unique id. Adding and removing by index is O(1).
//!
 //! # Examples
//!
 //! ```
 //! # use id_map::IdMap;
 //! #
 //! assert!(!map.contains(red_id));
 //! assert_eq!(map[blue_id], "blue");
 //! ```
//!
 //! [`IdMap`]: struct.IdMap.html
 
#![deny(missing_docs, missing_debug_implementations, unsafe_code)]
 
 extern crate id_set;
 
 
 use std::iter::FromIterator;
 use std::ops::{Index, IndexMut};
use std::{cmp, fmt, mem};
use std::{slice, vec};
 
 use id_set::IdSet;
 
 /// A container that gives each item a unique id. Internally all elements are stored contiguously.
#[derive(Clone)]
 pub struct IdMap<T> {
     // The set of valid indices for values.
     ids: IdSet,
     // The buffer of values. Indices not in ids are invalid.
    values: Vec<Option<T>>,
    // The smallest empty space in the vector of values, or values.len() if no space is left.
     space: Id,
 }
 
     #[inline]
     /// Removes all values from the map.
     pub fn clear(&mut self) {
        self.drop_values();
         self.ids.clear();
     }
 
     /// Resizes the map to minimize allocated memory.
     pub fn shrink_to_fit(&mut self) {
         self.ids.shrink_to_fit();
        self.values.truncate(self.ids.capacity());
        self.values.shrink_to(self.ids.capacity());
     }
 
     #[inline]
     /// Inserts a value into an empty slot in the map and returns its id.
     pub fn insert(&mut self, val: T) -> Id {
         let id = self.space;
        if id == self.values.len() {
            self.values.resize_with(id + 1, Default::default);
         }
        self.values[id] = Some(val);
         self.ids.insert(id);
         self.find_space();
         id
             if id == self.space {
                 self.find_space();
             }
            if self.values.len() < id + 1 {
                self.values.resize_with(id + 1, Default::default);
             }
            self.values[id] = Some(val);
             None
         } else {
             // val was previously in the map
            Some(mem::replace(&mut self.values[id].as_mut().unwrap(), val))
         }
     }
 
     pub fn remove(&mut self, id: Id) -> Option<T> {
         if self.ids.remove(id) {
             self.space = cmp::min(self.space, id);
            self.values[id].take()
         } else {
             None
         }
     pub fn get_or_insert(&mut self, id: Id, val: T) -> &mut T {
         self.get_or_insert_with(id, || val)
     }

     #[inline]
     /// If the id has a value, returns it, otherwise inserts a new value with the provided closure.
     pub fn get_or_insert_with<F: FnOnce() -> T>(&mut self, id: Id, f: F) -> &mut T {
             if id == self.space {
                 self.find_space();
             }
            if self.values.len() < id + 1 {
                self.values.resize_with(id + 1, Default::default);
             }
            self.values[id] = Some(f());
         }

        self.values[id].as_mut().unwrap()
     }
 
     #[inline]
             if let Some(first) = iter.next() {
                 // Set iterators are increasing so we only need to change start once.
                 self.space = cmp::min(self.space, first);
                self.values[first] = None;
                 for id in iter {
                    self.values[id] = None;
                 }
             }
         }
         let values = &mut self.values;
         let space = &mut self.space;
         ids.retain(|id| {
            if pred(id, values[id].as_ref().unwrap()) {
                true
            } else {
                *space = cmp::min(*space, id);
                values[id] = None;
                false
             }
         })
     }
     /// Returns a reference to the value at the specified id if it is in the map.
     pub fn get(&self, id: Id) -> Option<&T> {
         if self.ids.contains(id) {
            Some(self.values[id].as_ref().unwrap())
         } else {
             None
         }
     /// Returns a mutable reference to the value at the specified id if it is in the map.
     pub fn get_mut(&mut self, id: Id) -> Option<&mut T> {
         if self.ids.contains(id) {
            Some(self.values[id].as_mut().unwrap())
         } else {
             None
         }
     pub fn values(&self) -> Values<T> {
         Values {
             ids: self.ids.iter(),
            values: &self.values,
         }
     }
 
     pub fn values_mut(&mut self) -> ValuesMut<T> {
         ValuesMut {
             ids: self.ids.iter(),
            prev: None,
            values: self.values.iter_mut(),
         }
     }
 
     pub fn iter(&self) -> Iter<T> {
         Iter {
             ids: self.ids.iter(),
            values: &self.values,
         }
     }
 
     pub fn iter_mut(&mut self) -> IterMut<T> {
         IterMut {
             ids: self.ids.iter(),
            prev: None,
            values: self.values.iter_mut(),
         }
     }
 
     #[inline]
     /// A consuming iterator over id-value pairs, in order of increasing id.
     pub fn into_iter(self) -> IntoIter<T> {
         IntoIter {
            ids: self.ids.into_iter(),
            prev: None,
            values: self.values.into_iter(),
         }
     }
 
             assert!(self.ids.contains(id));
         }
         assert!(!self.ids.contains(self.space));
        // values.len() should be an upper bound on ids.
         for id in &self.ids {
            assert!(id < self.values.len())
         }
     }
 
    /// Clear the values vec.
    fn drop_values(&mut self) {
         for id in &self.ids {
            self.values[id] = None;
         }
     }
 
    /// Find the next empty space after one has been filled.
     fn find_space(&mut self) {
         // Each id corresponds to an entry in the storage so ids can never fill up.
         self.space += 1;
     }
 }
 
 impl<T: fmt::Debug> fmt::Debug for IdMap<T> {
     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
         write!(f, "{{")?;
 
 impl<T: PartialEq> PartialEq for IdMap<T> {
     fn eq(&self, other: &Self) -> bool {
        self.ids == other.ids
            && self
                .ids
                .iter()
                .zip(&other.ids)
                .all(|(l, r)| self.values[l].as_ref().unwrap() == other.values[r].as_ref().unwrap())
     }
 }
 
 impl<T> FromIterator<T> for IdMap<T> {
     #[inline]
     fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let values = Vec::from_iter(iter.into_iter().map(Some));
        let space = values.len();
        let ids = IdSet::new_filled(values.len());
        IdMap { values, space, ids }
     }
 }
 
     #[inline]
     fn index(&self, id: Id) -> &Self::Output {
         assert!(self.ids.contains(id), "id {} out of bounds", id);
        self.values[id].as_ref().unwrap()
     }
 }
 
 impl<T> IndexMut<Id> for IdMap<T> {
     #[inline]
     fn index_mut(&mut self, id: Id) -> &mut Self::Output {
         assert!(self.ids.contains(id), "id {} out of bounds", id);
        self.values[id].as_mut().unwrap()
     }
 }
 
 /// An iterator over all values, in order of increasing id.
 pub struct Values<'a, T: 'a> {
     ids: id_set::Iter<'a>,
    values: &'a [Option<T>],
 }
 
 impl<'a, T: 'a> Iterator for Values<'a, T> {
     type Item = &'a T;
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        self.ids.next().map(|id| self.values[id].as_ref().unwrap())
     }
 
     #[inline]
 /// A mutable iterator over all values, in order of increasing id.
 pub struct ValuesMut<'a, T: 'a> {
     ids: id_set::Iter<'a>,
    prev: Option<Id>,
    values: slice::IterMut<'a, Option<T>>,
 }
 
 impl<'a, T: 'a> Iterator for ValuesMut<'a, T> {
     type Item = &'a mut T;
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.next()?;
        let n = match self.prev {
            Some(prev) => id - prev - 1,
            None => 0,
        };
        self.prev = Some(id);

        Some(self.values.nth(n).unwrap().as_mut().unwrap())
     }
 
     #[inline]
 /// An iterator over id-value pairs, in order of increasing id.
 pub struct Iter<'a, T: 'a> {
     ids: id_set::Iter<'a>,
    values: &'a [Option<T>],
 }
 
 impl<'a, T: 'a> Iterator for Iter<'a, T> {
     type Item = (Id, &'a T);
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        self.ids
            .next()
            .map(|id| (id, self.values[id].as_ref().unwrap()))
     }
 
     #[inline]
 /// A mutable iterator over id-value pairs, in order of increasing id.
 pub struct IterMut<'a, T: 'a> {
     ids: id_set::Iter<'a>,
    prev: Option<Id>,
    values: slice::IterMut<'a, Option<T>>,
 }
 
 impl<'a, T: 'a> Iterator for IterMut<'a, T> {
     type Item = (Id, &'a mut T);
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.next()?;
        let n = match self.prev {
            Some(prev) => id - prev - 1,
            None => 0,
        };
        self.prev = Some(id);

        Some((
            id,
            self.values.nth(n).unwrap().as_mut().expect("id not in map"),
        ))
     }
 
     #[inline]
 /// A consuming iterator over id-value pairs, in order of increasing id.
 pub struct IntoIter<T> {
     ids: id_set::IntoIter,
    prev: Option<Id>,
    values: vec::IntoIter<Option<T>>,
 }
 
 impl<T> Iterator for IntoIter<T> {
     type Item = (Id, T);
 
     #[inline]
     fn next(&mut self) -> Option<Self::Item> {
        let id = self.ids.next()?;
        let n = match self.prev {
            Some(prev) => id - prev - 1,
            None => 0,
        };
        self.prev = Some(id);

        Some((id, self.values.nth(n).unwrap().expect("id not in map")))
     }
 
     #[inline]
     fn size_hint(&self) -> (usize, Option<usize>) {
         self.ids.size_hint()
     }
}