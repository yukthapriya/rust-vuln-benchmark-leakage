// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0038 (FIXED)
// Crate: juniper
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: graphql-rust/juniper
// Commit: 180a513
// File: juniper/src/ast.rs


#[derive(Clone, Debug, PartialEq)]
 pub struct VariableDefinition<'a, S> {
    pub description: Option<Spanning<Cow<'a, str>>>,
     pub var_type: Spanning<Type<&'a str>>,
     pub default_value: Option<Spanning<InputValue<S>>>,
     pub directives: Option<Vec<Spanning<Directive<'a, S>>>>,
 #[expect(missing_docs, reason = "self-explanatory")]
 #[derive(Clone, Debug, PartialEq)]
 pub struct Operation<'a, S> {
    pub description: Option<Spanning<Cow<'a, str>>>,
     pub operation_type: OperationType,
     pub name: Option<Spanning<&'a str>>,
     pub variable_definitions: Option<Spanning<VariableDefinitions<'a, S>>>,
 #[derive(Clone, Debug, PartialEq)]
 pub struct Fragment<'a, S> {
     pub name: Spanning<&'a str>,
    pub description: Option<Spanning<Cow<'a, str>>>,
     pub type_condition: Spanning<&'a str>,
     pub directives: Option<Vec<Spanning<Directive<'a, S>>>>,
     pub selection_set: Vec<Selection<'a, S>>,
     Fragment(Spanning<Fragment<'a, S>>),
 }
 
impl<'a, S> Definition<'a, S> {
    /// Sets or resets the provided `description` for this [`Definition`].
    pub(crate) fn set_description(&mut self, description: Option<Spanning<Cow<'a, str>>>) {
        match self {
            Self::Operation(op) => op.item.description = description,
            Self::Fragment(frag) => frag.item.description = description,
        }
    }
}

 #[doc(hidden)]
 pub type Document<'a, S> = [Definition<'a, S>];
 #[doc(hidden)]