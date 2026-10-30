// REAL FIXED CODE
// RustSec ID: RUSTSEC-2024-0331 (FIXED)
// Crate: puccinier
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: catppuccin/toolbox
// Commit: 59bc0f6
// File: whiskers/src/frontmatter.rs


use detect_newline_style::LineEnding;
 use std::collections::HashMap;
 
 #[derive(Debug)]
     // we consider a template to possibly have frontmatter iff:
     // * line 0 is "---"
     // * there is another "---" on another line
    let template = template.trim_start();
    let eol = LineEnding::find(template, LineEnding::LF).to_string();
    let sep = "---".to_string() + &eol;
    if !template.starts_with(&sep) {
         return None;
     }
 
     template[sep.len()..]
        .split_once(&sep)
        .map(|(a, b)| (a.trim(), b))
 }