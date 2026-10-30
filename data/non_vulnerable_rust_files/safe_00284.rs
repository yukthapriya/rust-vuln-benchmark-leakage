// REAL FIXED CODE
// RustSec ID: RUSTSEC-2022-0037 (FIXED)
// Crate: async-graphql
// Type: unknown (FIXED)
// Extraction Method: commit_diff
// GitHub Repo: async-graphql/async-graphql
// Commit: 521769b
// File: src/schema.rs


model::__DirectiveLocation,
     parser::{
         parse_query,
        types::{
            Directive, DocumentOperations, ExecutableDocument, OperationType, Selection,
            SelectionSet,
        },
         Positioned,
     },
     registry::{MetaDirective, MetaInputValue, Registry, SDLExportOptions},
     types::QueryRoot,
     validation::{check_rules, ValidationMode},
     BatchRequest, BatchResponse, CacheControl, ContextBase, EmptyMutation, EmptySubscription,
    InputType, ObjectType, OutputType, QueryEnv, Request, Response, ServerError, ServerResult,
    SubscriptionType, Variables, ID,
 };
 
 /// Introspection mode
     data: Data,
     complexity: Option<usize>,
     depth: Option<usize>,
    recursive_depth: usize,
     extensions: Vec<Box<dyn ExtensionFactory>>,
     custom_directives: HashMap<&'static str, Box<dyn CustomDirectiveFactory>>,
 }
         self
     }
 
    /// Set the maximum recursive depth a query can have. (default: 256)
    ///
    /// If the value is too large, stack overflow may occur, usually `256` is
    /// enough.
    #[must_use]
    pub fn limit_recursive_depth(mut self, depth: usize) -> Self {
        self.recursive_depth = depth;
        self
    }

     /// Add an extension to the schema.
     ///
     /// # Examples
             subscription: self.subscription,
             complexity: self.complexity,
             depth: self.depth,
            recursive_depth: self.recursive_depth,
             extensions: self.extensions,
             env: SchemaEnv(Arc::new(SchemaEnvInner {
                 registry: self.registry,
     pub(crate) subscription: Subscription,
     pub(crate) complexity: Option<usize>,
     pub(crate) depth: Option<usize>,
    pub(crate) recursive_depth: usize,
     pub(crate) extensions: Vec<Box<dyn ExtensionFactory>>,
     pub(crate) env: SchemaEnv,
 }
             data: Default::default(),
             complexity: None,
             depth: None,
            recursive_depth: 256,
             extensions: Default::default(),
             custom_directives: Default::default(),
         }
         let mut document = {
             let query = &request.query;
             let parsed_doc = request.parsed_query.take();
            let recursive_depth = self.recursive_depth;
             let fut_parse = async move {
                let doc = match parsed_doc {
                    Some(parsed_doc) => parsed_doc,
                    None => parse_query(query)?,
                };
                check_recursive_depth(&doc, recursive_depth)?;
                Ok(doc)
             };
             futures_util::pin_mut!(fut_parse);
             extensions
         }
     }
 }

fn check_recursive_depth(doc: &ExecutableDocument, max_depth: usize) -> ServerResult<()> {
    fn check_selection_set(
        doc: &ExecutableDocument,
        selection_set: &Positioned<SelectionSet>,
        current_depth: usize,
        max_depth: usize,
    ) -> ServerResult<()> {
        if current_depth > max_depth {
            return Err(ServerError::new(
                format!(
                    "The recursion depth of the query cannot be greater than `{}`",
                    max_depth
                ),
                Some(selection_set.pos),
            ));
        }

        for selection in &selection_set.node.items {
            match &selection.node {
                Selection::Field(field) => {
                    if !field.node.selection_set.node.items.is_empty() {
                        check_selection_set(
                            doc,
                            &field.node.selection_set,
                            current_depth + 1,
                            max_depth,
                        )?;
                    }
                }
                Selection::FragmentSpread(fragment_spread) => {
                    if let Some(fragment) =
                        doc.fragments.get(&fragment_spread.node.fragment_name.node)
                    {
                        check_selection_set(
                            doc,
                            &fragment.node.selection_set,
                            current_depth + 1,
                            max_depth,
                        )?;
                    }
                }
                Selection::InlineFragment(inline_fragment) => {
                    check_selection_set(
                        doc,
                        &inline_fragment.node.selection_set,
                        current_depth + 1,
                        max_depth,
                    )?;
                }
            }
        }

        Ok(())
    }

    for (_, operation) in doc.operations.iter() {
        check_selection_set(doc, &operation.node.selection_set, 0, max_depth)?;
    }

    Ok(())
}