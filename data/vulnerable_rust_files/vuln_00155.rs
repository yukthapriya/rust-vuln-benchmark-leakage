// REAL VULNERABLE CODE
// RustSec ID: RUSTSEC-2020-0152
// CVE: CVE-2020-36472
// CWE: N/A
// Crate: max7301
// Type: race_condition
// Severity: medium
// Source: https://crates.io/crates/max7301
// Extraction Method: repo_search
// GitHub Repo: edarc/max7301
// Commit: 0a1da87
// File: src/config.rs


///     .unwrap();
 /// ```
 #[must_use = "Configuration changes are not applied unless committed"]
pub struct Configurator<'e, EI: ExpanderInterface> {
     expander: &'e mut Expander<EI>,
     expander_config_dirty: bool,
     banks: [BankConfig; 7],
 }
 
impl<'e, EI: ExpanderInterface> Configurator<'e, EI> {
     pub(crate) fn new(expander: &'e mut Expander<EI>) -> Self {
         Self {
             expander,