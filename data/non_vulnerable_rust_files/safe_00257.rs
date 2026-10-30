// REAL FIXED CODE
// RustSec ID: RUSTSEC-2021-0142 (FIXED)
// Crate: dotenv_codegen
// Type: unknown (FIXED)
// Extraction Method: repo_search
// GitHub Repo: dotenv-rs/dotenv
// Commit: 8c2ba38
// File: dotenv/src/bin/dotenv.rs


use clap::{App, AppSettings, Arg};
 use std::os::unix::process::CommandExt;
use std::process::{exit, Command};
 
 macro_rules! die {
     ($fmt:expr) => ({
         .setting(AppSettings::AllowExternalSubcommands)
         .setting(AppSettings::ArgRequiredElseHelp)
         .setting(AppSettings::UnifiedHelpMessage)
        .arg(
            Arg::with_name("FILE")
                .short("f")
                .long("file")
                .takes_value(true)
                .help("Use a specific .env file (defaults to .env)"),
        )
         .get_matches();
 
     match matches.value_of("FILE") {
         None => dotenv::dotenv(),
         Some(file) => dotenv::from_filename(file),
    }
    .unwrap_or_else(|e| die!("error: failed to load environment: {}", e));
 
     let mut command = match matches.subcommand() {
         (name, Some(matches)) => {
            let args = matches
                .values_of("")
                 .map(|v| v.collect())
                 .unwrap_or(Vec::new());
 
             make_command(name, args)
        }
         _ => die!("error: missing required argument <COMMAND>"),
     };