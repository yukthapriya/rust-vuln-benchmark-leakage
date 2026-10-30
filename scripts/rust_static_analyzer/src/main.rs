use walkdir::WalkDir;
use serde::Serialize;
use syn::{File, visit::Visit};
use regex::Regex;
use std::fs;

#[derive(Debug, Serialize)]
struct StaticFeatures {
    file_path: String,
    n_unsafe: usize,
    n_raw_ptr: usize,
    n_lifetimes: usize,
    n_loops: usize,
    n_functions: usize,
    n_branches: usize,
    n_lines: usize,
    n_chars: usize,
}

struct FeatureVisitor {
    n_unsafe: usize,
    n_raw_ptr: usize,
    n_lifetimes: usize,
    n_loops: usize,
    n_functions: usize,
    n_branches: usize,
}

impl<'ast> Visit<'ast> for FeatureVisitor {
    fn visit_item_fn(&mut self, _: &'ast syn::ItemFn) {
        self.n_functions += 1;
    }

    fn visit_expr_loop(&mut self, _: &'ast syn::ExprLoop) {
        self.n_loops += 1;
    }

    fn visit_expr_for_loop(&mut self, _: &'ast syn::ExprForLoop) {
        self.n_loops += 1;
    }

    fn visit_expr_while(&mut self, _: &'ast syn::ExprWhile) {
        self.n_loops += 1;
    }

    fn visit_expr_if(&mut self, _: &'ast syn::ExprIf) {
        self.n_branches += 1;
    }

    fn visit_expr_match(&mut self, _: &'ast syn::ExprMatch) {
        self.n_branches += 1;
    }

    fn visit_expr_unsafe(&mut self, _: &'ast syn::ExprUnsafe) {
        self.n_unsafe += 1;
    }
}

fn analyze_file(path: &str) -> Option<StaticFeatures> {
    let content = fs::read_to_string(path).ok()?;

    let syntax: File = syn::parse_file(&content).ok()?;

    let mut visitor = FeatureVisitor {
        n_unsafe: 0,
        n_raw_ptr: 0,
        n_lifetimes: 0,
        n_loops: 0,
        n_functions: 0,
        n_branches: 0,
    };

    visitor.visit_file(&syntax);

    let raw_ptr_re = Regex::new(r"\*const|\*mut").unwrap();
    let lifetime_re = Regex::new(r"'[a-zA-Z_]\w*").unwrap();

    let n_raw_ptr = raw_ptr_re.find_iter(&content).count();
    let n_lifetimes = lifetime_re.find_iter(&content).count();

    let n_lines = content.lines().count();
    let n_chars = content.len();

    Some(StaticFeatures {
        file_path: path.to_string(),
        n_unsafe: visitor.n_unsafe,
        n_raw_ptr,
        n_lifetimes,
        n_loops: visitor.n_loops,
        n_functions: visitor.n_functions,
        n_branches: visitor.n_branches,
        n_lines,
        n_chars,
    })
}

fn main() {
    let root = ".."; // <-- CHANGE THIS to your Rust files folder

    let mut results = Vec::new();

    for entry in WalkDir::new(root).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path().to_string_lossy().to_string();
        if path.ends_with(".rs") && !path.contains("rust_static_analyzer") {
        // if path.ends_with(".rs") {
            if let Some(features) = analyze_file(&path) {
                results.push(features);
            }
        }
    }

    println!("{}", serde_json::to_string_pretty(&results).unwrap());
}
