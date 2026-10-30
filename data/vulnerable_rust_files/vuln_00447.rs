// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0388
// CVE: N/A
// CWE: N/A
// Crate: derivative
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/derivative
// Extraction Method: repo_search
// GitHub Repo: mcarton/rust-derivative
// Commit: 725e733
// File: src/cmp.rs


quote! {
         #[allow(unused_qualifications)]
         impl #impl_generics #partial_eq_trait_path for #name #ty_generics #where_clause {
             fn eq(&self, other: &Self) -> bool {
                 #discriminant_cmp && #match_fields
 
     quote! {
         #[allow(unused_qualifications)]
         impl #impl_generics #partial_ord_trait_path for #name #ty_generics #where_clause {
             fn partial_cmp(&self, other: &Self) -> #option_path<#ordering_path> {
                 match *self {
 
     quote! {
         #[allow(unused_qualifications)]
         impl #impl_generics #ord_trait_path for #name #ty_generics #where_clause {
             fn cmp(&self, other: &Self) -> #ordering_path {
                 match *self {