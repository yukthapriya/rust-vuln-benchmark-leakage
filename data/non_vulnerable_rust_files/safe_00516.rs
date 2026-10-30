// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0048 (FIXED)
// Crate: tsify-next
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: siefkenj/tsify
// Commit: 9cb9563
// File: tests/expand/borrow.expanded.rs


Ok(js) => js.into_abi(),
                 Err(err) => {
                     let loc = core::panic::Location::caller();
                    let msg = ::alloc::__export::must_use({
                         let res = ::alloc::fmt::format(
                             format_args!(
                                 "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                 .file(), loc.line(), loc.column(),
                             ),
                         );
                         res
                    });
                     {
                         #[cold]
                         #[track_caller]
                 Ok(js) => js.into(),
                 Err(err) => {
                     let loc = core::panic::Location::caller();
                    let msg = ::alloc::__export::must_use({
                         let res = ::alloc::fmt::format(
                             format_args!(
                                 "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                 .file(), loc.line(), loc.column(),
                             ),
                         );
                         res
                    });
                     {
                         #[cold]
                         #[track_caller]
                     Ok(js) => js.into(),
                     Err(err) => {
                         let loc = core::panic::Location::caller();
                        let msg = ::alloc::__export::must_use({
                             let res = ::alloc::fmt::format(
                                 format_args!(
                                     "(Converting type failed) {0} ({1}:{2}:{3})", err, loc
                                     .file(), loc.line(), loc.column(),
                                 ),
                             );
                             res
                        });
                         {
                             #[cold]
                             #[track_caller]