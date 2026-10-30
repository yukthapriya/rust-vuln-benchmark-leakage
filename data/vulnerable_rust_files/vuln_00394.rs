// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2024-0009
// CVE: CVE-2024-23644
// CWE: N/A
// Crate: trillium-http
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/trillium-http
// Extraction Method: repo_search
// GitHub Repo: trillium-rs/trillium
// Commit: a108c8b
// File: http/src/upgrade.rs


impl<Transport> Upgrade<Transport> {
     /// see [`request_headers`]
    #[deprecated = "use Upgrade::request_headers"]
     pub fn headers(&self) -> &Headers {
         &self.request_headers
     }
 
    /// read-only access to the request headers
    pub fn request_headers(&self) -> &Headers {
        &self.request_headers
    }

    /// mutable access to headers
    pub fn request_headers_mut(&mut self) -> &mut Headers {
        &mut self.request_headers
    }

     /// the http request path up to but excluding any query component
     pub fn path(&self) -> &str {
         match self.path.split_once('?') {
 
     /// retrieves the query component of the path
     pub fn querystring(&self) -> &str {
        match self.path.split_once('?') {
            Some((_, query)) => query,
            None => "",
        }
     }
 
     /// the http method