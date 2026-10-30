// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0070 (FIXED)
// Crate: nalgebra
// Type: unknown (FIXED)
// Extraction Method: crate_versions
// GitHub Repo: nalgebra
// Commit: N/A
// File: lib.rs


#![feature(bench_black_box)]
#![allow(unused_macros)]

extern crate nalgebra as na;
extern crate rand_package as rand;

#[macro_use]
extern crate criterion;

use na::DMatrix;
use rand::Rng;
use rand_isaac::IsaacRng;

pub mod core;
pub mod geometry;
pub mod linalg;

fn reproductible_dmatrix(nrows: usize, ncols: usize) -> DMatrix<f64> {
    use rand::SeedableRng;
    let mut rng = IsaacRng::seed_from_u64(0);
    DMatrix::<f64>::from_fn(nrows, ncols, |_, _| rng.gen())
}

criterion_main!(
    core::matrix,
    core::vector,
    geometry::quaternion,
    linalg::bidiagonal,
    linalg::cholesky,
    linalg::full_piv_lu,
    linalg::hessenberg,
    linalg::lu,
    linalg::qr,
    linalg::schur,
    linalg::solve,
    linalg::svd,
    linalg::symmetric_eigen,
);
