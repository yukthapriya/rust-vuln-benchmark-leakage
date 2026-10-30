// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0009 (FIXED)
// Crate: trillium-http
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: trillium-rs/trillium
// Commit: a108c8b
// File: http/src/upgrade.rs


impl<Transport> Upgrade<Transport> {
     /// see [`request_headers`]
    #[deprecated = "directly access the request_headers field"]
     pub fn headers(&self) -> &Headers {
         &self.request_headers
     }
 
     /// the http request path up to but excluding any query component
     pub fn path(&self) -> &str {
         match self.path.split_once('?') {
 
     /// retrieves the query component of the path
     pub fn querystring(&self) -> &str {
        self.path
            .split_once('?')
            .map(|(_, query)| query)
            .unwrap_or_default()
     }
 
     /// the http method