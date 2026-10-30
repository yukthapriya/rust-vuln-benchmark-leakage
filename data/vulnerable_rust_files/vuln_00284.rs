// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2022-0037
// CVE: N/A
// CWE: N/A
// Crate: async-graphql
// Type: unknown
// Severity: medium
// Source: https://crates.io/crates/async-graphql
// Extraction Method: commit_diff
// GitHub Repo: async-graphql/async-graphql
// Commit: 521769b
// File: src/schema.rs


model::__DirectiveLocation,
     parser::{
         parse_query,
        types::{Directive, DocumentOperations, OperationType, Selection, SelectionSet},
         Positioned,
     },
     registry::{MetaDirective, MetaInputValue, Registry, SDLExportOptions},
     types::QueryRoot,
     validation::{check_rules, ValidationMode},
     BatchRequest, BatchResponse, CacheControl, ContextBase, EmptyMutation, EmptySubscription,
    InputType, ObjectType, OutputType, QueryEnv, Request, Response, ServerError, SubscriptionType,
    Variables, ID,
 };
 
 /// Introspection mode
     data: Data,
     complexity: Option<usize>,
     depth: Option<usize>,
     extensions: Vec<Box<dyn ExtensionFactory>>,
     custom_directives: HashMap<&'static str, Box<dyn CustomDirectiveFactory>>,
 }
         self
     }
 
     /// Add an extension to the schema.
     ///
     /// # Examples
             subscription: self.subscription,
             complexity: self.complexity,
             depth: self.depth,
             extensions: self.extensions,
             env: SchemaEnv(Arc::new(SchemaEnvInner {
                 registry: self.registry,
     pub(crate) subscription: Subscription,
     pub(crate) complexity: Option<usize>,
     pub(crate) depth: Option<usize>,
     pub(crate) extensions: Vec<Box<dyn ExtensionFactory>>,
     pub(crate) env: SchemaEnv,
 }
             data: Default::default(),
             complexity: None,
             depth: None,
             extensions: Default::default(),
             custom_directives: Default::default(),
         }
         let mut document = {
             let query = &request.query;
             let parsed_doc = request.parsed_query.take();
             let fut_parse = async move {
                match parsed_doc {
                    Some(parsed_doc) => Ok(parsed_doc),
                    None => parse_query(query).map_err(Into::into),
                }
             };
             futures_util::pin_mut!(fut_parse);
             extensions
         }
     }
 }