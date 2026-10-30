// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0031 (FIXED)
// Crate: nano_arena
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: bennetthardwick/nano-arena
// Commit: 6b83f9d
// File: src/lib.rs


&'a mut self,
         selected: I,
     ) -> Option<(&mut T, ArenaSplit<'a, T>)> {
        let selected = selected.borrow();

        if let Some(value) = self.get_mut(selected) {
             Some((
                 unsafe { (value as *mut T).as_mut().unwrap() },
                 ArenaSplit {
                    selected: selected.clone(),
                     arena: self,
                     __type: Default::default(),
                 },
 #[cfg(test)]
 mod tests {
     use super::*;
    use std::cell::Cell;
 
     fn setup_arena() -> (Arena<String>, Idx, Idx, Idx, Idx) {
         let mut arena = Arena::new();
 
         assert_eq!(format!("{:?}", john), "Removed Idx ( 0 )");
     }

    #[test]
    fn compare_should_work_for_idx() {
        let mut arena = Arena::new();
        let left = arena.alloc(1);
        let right = left.clone();

        assert!(left.borrow() == right.borrow());
    }

    #[test]
    fn splitting_should_be_safe() {
        // A bug described here (https://github.com/bennetthardwick/nano-arena/issues/1) meant that
        // multiple mutable references could be handed out.

        struct ToggleIdx {
            first: Idx,
            second: Idx,
            state: Cell<bool>,
        }

        impl Borrow<Idx> for ToggleIdx {
            fn borrow(&self) -> &Idx {
                self.state.set(!self.state.get());

                if self.state.get() {
                    &self.first
                } else {
                    &self.second
                }
            }
        }

        let mut arena = Arena::new();

        let first = arena.alloc(1);
        let second = arena.alloc(2);

        let toggle_idx = ToggleIdx {
            first: first.clone(),
            second: second.clone(),
            state: Cell::new(false),
        };

        let (first_mut_ref, mut split_arena) = arena.split_at(toggle_idx).unwrap();
        let second_mut_ref = split_arena.get_mut(&first);

        drop(first_mut_ref);
        assert!(second_mut_ref.is_none());
    }
 }