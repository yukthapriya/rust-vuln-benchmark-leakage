// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2019-0001
// CVE: CVE-2019-15542
// CWE: N/A
// Crate: ammonia
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/ammonia
// Extraction Method: repo_search
// GitHub Repo: rust-ammonia/ammonia
// Commit: 6aa8a12
// File: src/lib.rs


let parent = node.parent
                 .replace(None).expect("a node in the DOM will have a parent, except the root, which is not processed")
                 .upgrade().expect("a node's parent will be pointed to by its parent (or the root pointer), and will not be dropped");
            if self.clean_node_content(&node) {
                 removed.push(node);
                 continue;
             }
            let pass_clean = self.clean_child(&mut node);
            let pass = pass_clean && self.check_expected_namespace(&parent, &node);
             if pass {
                 self.adjust_node_attributes(&mut node, &link_rel, self.id_prefix);
                 dom.append(&parent.clone(), NodeOrText::AppendNode(node.clone()));
             matches!(
                 &*parent.local,
                 "mi" | "mo" | "mn" | "ms" | "mtext" | "annotation-xml"
            )
         // The only way to switch from svg to mathml/html is with an html integration point
         } else if parent.ns == ns!(svg) && child.ns != ns!(svg) {
             // https://html.spec.whatwg.org/#svg-0
             matches!(&*parent.local, "foreignObject")
         } else if child.ns == ns!(svg) {
             is_svg_tag(&child.local)
         } else if child.ns == ns!(mathml) {
             is_mathml_tag(&child.local)
         } else if child.ns == ns!(html) {
            (!is_svg_tag(&child.local) && !is_mathml_tag(&child.local))
                || matches!(
                    &*child.local,
                    "title" | "style" | "font" | "a" | "script" | "span"
                )
         } else {
             // There are no other supported ways to switch namespace
             parent.ns == child.ns
         || (element == "video" && attr == "poster")
 }
 
 /// Given an element name, check if it's SVG
 fn is_svg_tag(element: &str) -> bool {
     // https://svgwg.org/svg2-draft/eltindex.html
         // https://github.com/cure53/DOMPurify/pull/495
         let fragment = r##"<svg><iframe><a title="</iframe><img src onerror=alert(1)>">test"##;
         let result = String::from(Builder::new().add_tags(&["iframe"]).clean(fragment));
        assert_eq!(result.to_string(), "test");
 
         let fragment = "<svg><iframe>remove me</iframe></svg><iframe>keep me</iframe>";
         let result = String::from(Builder::new().add_tags(&["iframe"]).clean(fragment));
        assert_eq!(result.to_string(), "remove me<iframe>keep me</iframe>");
 
         let fragment = "<svg><a>remove me</a></svg><iframe>keep me</iframe>";
         let result = String::from(Builder::new().add_tags(&["iframe"]).clean(fragment));
        assert_eq!(result.to_string(), "remove me<iframe>keep me</iframe>");
 
         let fragment = "<svg><a>keep me</a></svg><iframe>keep me</iframe>";
         let result = String::from(Builder::new().add_tags(&["iframe", "svg"]).clean(fragment));
         );
     }
 
     #[test]
     fn ns_mathml() {
         // https://github.com/cure53/DOMPurify/pull/495
         );
     }
 
     #[test]
     fn xml_processing_instruction() {
         // https://blog.slonser.info/posts/dompurify-node-type-confusion/