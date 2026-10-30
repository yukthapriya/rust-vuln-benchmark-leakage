// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0087 (FIXED)
// Crate: slack-morphism
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: abdolence/slack-morphism-rust
// Commit: 65ef9fa
// File: src/api/webhook.rs


rate_control_params: Some(&POST_WEBHOOK_SPECIAL_LIMIT_RATE_CTL),
             token: None,
             tracing_span: &http_webhook_span,
            is_sensitive_url: true,
         };
 
         self.http_api