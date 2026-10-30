// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0087
// CVE: CVE-2022-39292
// CWE: N/A
// Crate: slack-morphism
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/slack-morphism
// Extraction Method: commit_diff
// GitHub Repo: abdolence/slack-morphism-rust
// Commit: 65ef9fa
// File: src/api/webhook.rs


rate_control_params: Some(&POST_WEBHOOK_SPECIAL_LIMIT_RATE_CTL),
             token: None,
             tracing_span: &http_webhook_span,
         };
 
         self.http_api