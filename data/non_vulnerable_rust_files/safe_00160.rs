// REAL FIXED CODE
// RustSec ID: RUSTSEC-2020-0164 (FIXED)
// Crate: cell-project
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: RustyYato/cell-project
// Commit: b89fe8e
// File: src/lib.rs


ref cell => unsafe {
                 let cell: &$crate::macros::Cell<$type> = cell;
                 let ptr = cell.as_ptr();
                let $type { $field: _, .. } = &mut *ptr;
                let field_ptr = $crate::macros::ptr::addr_of_mut!((*ptr).$field);
                $crate::macros::project_unchecked(cell, field_ptr)
             },
         }
     };