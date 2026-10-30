// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0037
// CVE: CVE-2020-35889
// CWE: N/A
// Crate: crayon
// Type: memory_safety
// Severity: medium
// Source: https://crates.io/crates/crayon
// Extraction Method: repo_search
// GitHub Repo: shawnscode/crayon
// Commit: 48d4e87
// File: modules/world/src/scene.rs


/// it traverses the hierarchy like a path name.
     #[inline]
     pub fn find<N: AsRef<str>>(&self, name: N) -> Option<Entity> {
        let mut components = name.as_ref().trim_left_matches('/').split('/');
         if let Some(first) = components.next() {
             for &v in &self.nodes.roots {
                 if let Some(n) = self.tags.name(v) {
     /// If no Entity with name can be found, None is returned. If name contains a '/' character,
     /// it traverses the hierarchy like a path name.
     pub fn find_from<N: AsRef<str>>(&self, root: Entity, name: N) -> Option<Entity> {
        let mut components = name.as_ref().trim_left_matches('/').split('/');
         let mut iter = root;
 
         while let Some(component) = components.next() {