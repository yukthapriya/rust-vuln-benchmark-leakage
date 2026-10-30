// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0164
// CVE: N/A
// CWE: N/A
// Crate: cell-project
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/cell-project
// Extraction Method: repo_search
// GitHub Repo: RustyYato/cell-project
// Commit: b89fe8e
// File: src/lib.rs


ref cell => unsafe {
                 let cell: &$crate::macros::Cell<$type> = cell;
                 let ptr = cell.as_ptr();
                let $type { $field: field, .. } = &mut *ptr;
                let field: *mut _ = field;
                let offset = (field as *const () as usize) - (ptr as *const () as usize);
                $crate::macros::project_unchecked(cell, field, offset)
            },
        }
    };
}

/// project through a shared mutable reference `&Cell`
///
/// see the crate docs for more information
#[macro_export]
#[cfg(any(feature = "nightly", doc))]
macro_rules! nightly_cell_project {
    ($type:path, $ident:ident.$field:tt) => {
        match $ident {
            ref cell => unsafe {
                let cell: &$crate::macros::Cell<$type> = cell;
                let ptr = cell.as_ptr();
                let $type { $field: field, .. } = &*ptr;
                $crate::macros::nightly_project_unchecked(cell, &raw mut (*ptr).$field)
             },
         }
     };