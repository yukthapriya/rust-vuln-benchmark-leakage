// REAL FIXED CODE
// RustSec ID: RUSTSEC-2025-0066 (FIXED)
// Crate: google-apis-common
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: Byron/google-apis-rs
// Commit: d7b09d6
// File: gen/gmail1/src/api.rs


/// // Provide your own `AuthenticatorDelegate` to adjust the way it operates and get feedback about
 /// // what's going on. You probably want to bring in your own `TokenStorage` to persist tokens and
 /// // retrieve them from storage.
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 ///     secret,
 ///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
 /// ).build().await.unwrap();
 ///
 /// let client = hyper_util::client::legacy::Client::builder(
 ///         .with_native_roots()
 ///         .unwrap()
 ///         .https_or_http()
///         .enable_http2()
 ///         .build()
 /// );
 /// let mut hub = Gmail::new(client, auth);
         Gmail {
             client,
             auth: Box::new(auth),
            _user_agent: "google-api-rust-client/7.0.0".to_string(),
             _base_url: "https://gmail.googleapis.com/".to_string(),
             _root_url: "https://gmail.googleapis.com/".to_string(),
         }
     }
 
     /// Set the user-agent header field to use in all requests to the server.
    /// It defaults to `google-api-rust-client/7.0.0`.
     ///
     /// Returns the previously set user-agent.
     pub fn user_agent(&mut self, agent_name: String) -> String {
 /// use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// let secret: yup_oauth2::ApplicationSecret = Default::default();
/// let connector = hyper_rustls::HttpsConnectorBuilder::new()
///     .with_native_roots()
///     .unwrap()
///     .https_only()
///     .enable_http2()
///     .build();
///
/// let executor = hyper_util::rt::TokioExecutor::new();
/// let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 ///     secret,
 ///     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
///     yup_oauth2::client::CustomHyperClientBuilder::from(
///         hyper_util::client::legacy::Client::builder(executor).build(connector),
///     ),
 /// ).build().await.unwrap();
 ///
 /// let client = hyper_util::client::legacy::Client::builder(
 ///         .with_native_roots()
 ///         .unwrap()
 ///         .https_or_http()
///         .enable_http2()
 ///         .build()
 /// );
 /// let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
         let mut url = self.hub._base_url.clone()
             + "gmail/v1/users/{userId}/messages/{messageId}/attachments/{id}";
         if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
         }
 
         #[allow(clippy::single_element_loop)]
     /// Identifies the authorization scope for the method you are building.
     ///
     /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
     ///
     /// The `scope` will be added to a set of scopes. This is important as one can maintain access
     /// tokens for more than one scope.
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
         params.push("alt", "json");
         let mut url = self.hub._base_url.clone() + "gmail/v1/users/{userId}/messages/{id}";
         if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
         }
 
         #[allow(clippy::single_element_loop)]
     /// Identifies the authorization scope for the method you are building.
     ///
     /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
     ///
     /// The `scope` will be added to a set of scopes. This is important as one can maintain access
     /// tokens for more than one scope.
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
         params.push("alt", "json");
         let mut url = self.hub._base_url.clone() + "gmail/v1/users/{userId}/threads/{id}";
         if self._scopes.is_empty() {
            self._scopes.insert(Scope::Readonly.as_ref().to_string());
         }
 
         #[allow(clippy::single_element_loop)]
     /// Identifies the authorization scope for the method you are building.
     ///
     /// Use this method to actively specify which scope should be used, instead of the default [`Scope`] variant
    /// [`Scope::Readonly`].
     ///
     /// The `scope` will be added to a set of scopes. This is important as one can maintain access
     /// tokens for more than one scope.
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);
 /// # use gmail1::{Gmail, FieldMask, hyper_rustls, hyper_util, yup_oauth2};
 ///
 /// # let secret: yup_oauth2::ApplicationSecret = Default::default();
/// # let connector = hyper_rustls::HttpsConnectorBuilder::new()
/// #     .with_native_roots()
/// #     .unwrap()
/// #     .https_only()
/// #     .enable_http2()
/// #     .build();
///
/// # let executor = hyper_util::rt::TokioExecutor::new();
/// # let auth = yup_oauth2::InstalledFlowAuthenticator::with_client(
 /// #     secret,
 /// #     yup_oauth2::InstalledFlowReturnMethod::HTTPRedirect,
/// #     yup_oauth2::client::CustomHyperClientBuilder::from(
/// #         hyper_util::client::legacy::Client::builder(executor).build(connector),
/// #     ),
 /// # ).build().await.unwrap();
 ///
 /// # let client = hyper_util::client::legacy::Client::builder(
 /// #         .with_native_roots()
 /// #         .unwrap()
 /// #         .https_or_http()
/// #         .enable_http2()
 /// #         .build()
 /// # );
 /// # let mut hub = Gmail::new(client, auth);