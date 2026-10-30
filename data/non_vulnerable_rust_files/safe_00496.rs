// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0020 (FIXED)
// Crate: pyo3
// Type: overflow (FIXED)
// Extraction Method: repo_search
// GitHub Repo: PyO3/pyo3
// Commit: 5d55fe5
// File: pyo3-macros-backend/src/module.rs


let gil_used = options.gil_used.is_some_and(|op| op.value.value);
 
     let initialization = module_initialization(
        &full_name,
         &name,
         ctx,
         quote! { __pyo3_pymodule },
     let gil_used = options.gil_used.is_some_and(|op| op.value.value);
 
     let initialization = module_initialization(
        &name.to_string(),
         &name,
         ctx,
         quote! { ModuleExec::__pyo3_module_exec },
 }
 
 fn module_initialization(
    full_name: &str,
     name: &syn::Ident,
     ctx: &Ctx,
     module_exec: TokenStream,
 ) -> TokenStream {
     let Ctx { pyo3_path, .. } = ctx;
     let pyinit_symbol = format!("PyInit_{name}");
    let pyo3_name = LitCStr::new(&CString::new(full_name).unwrap(), Span::call_site());
 
     let mut result = quote! {
         #[doc(hidden)]