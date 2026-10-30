// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0038
// CVE: CVE-2022-31173
// CWE: N/A
// Crate: juniper
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/juniper
// Extraction Method: repo_search
// GitHub Repo: graphql-rust/juniper
// Commit: 180a513
// File: juniper/src/ast.rs


#[derive(Clone, Debug, PartialEq)]
 pub struct VariableDefinition<'a, S> {
     pub var_type: Spanning<Type<&'a str>>,
     pub default_value: Option<Spanning<InputValue<S>>>,
     pub directives: Option<Vec<Spanning<Directive<'a, S>>>>,
 #[expect(missing_docs, reason = "self-explanatory")]
 #[derive(Clone, Debug, PartialEq)]
 pub struct Operation<'a, S> {
     pub operation_type: OperationType,
     pub name: Option<Spanning<&'a str>>,
     pub variable_definitions: Option<Spanning<VariableDefinitions<'a, S>>>,
 #[derive(Clone, Debug, PartialEq)]
 pub struct Fragment<'a, S> {
     pub name: Spanning<&'a str>,
     pub type_condition: Spanning<&'a str>,
     pub directives: Option<Vec<Spanning<Directive<'a, S>>>>,
     pub selection_set: Vec<Selection<'a, S>>,
     Fragment(Spanning<Fragment<'a, S>>),
 }
 
 #[doc(hidden)]
 pub type Document<'a, S> = [Definition<'a, S>];
 #[doc(hidden)]