// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0403
// CVE: N/A
// CWE: N/A
// Crate: js-sandbox
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/js-sandbox
// Extraction Method: repo_search
// GitHub Repo: Bromeon/js-sandbox
// Commit: 7923cd3
// File: js-sandbox/src/script.rs


}
 
 	fn create_script(js_code: &str, js_filename: &str) -> Result<Self, AnyError> {
		let ext = Extension::builder("script").ops(vec![(op_return::decl())]).build();
 
 		let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
 			module_loader: Some(Rc::new(deno_core::FsModuleLoader)),