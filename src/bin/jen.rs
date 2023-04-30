//! Jen is a utility to generate JSON documents from templates.
//!
//! This module provides a very easy and quick way to generate random
//! data using a simple command line API. Jen was written because there
//! are barely any good tools for this purposes which work in a terminal.
//! Most of the popular options are available via a browser, but this
//! severely limits the amount of data you can feasibly generate.
//!
//! The aim of this tool is to be sufficiently fast, whilst providing
//! various options to make generation more convenient. It was written
//! to scratch a personal itch, but there's no reason why new features
//! cannot be added if they're requested.
//!
//! Most of the underlying tooling is provided via the `Tera` crate
//! for templating, and the `fake` crate for data construction. Jen
//! itself is simply a binding around these two crates to provide a
//! convenience bridge between them. Go check them out! If you want
//! more than a CLI, you can also use `jen` programmatically.
#![doc(html_root_url = "https://docs.rs/jen/1.5.0")]
use clap::{Arg, Command};
use serde_json::Value;

use jen::error::Error;
use jen::generator::Generator;

use std::io::{self, Write};

/// Entry point of Jen.
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

/// Executes the main tooling of Jen.
fn run() -> Result<(), Error> {
    // parse the arguments from the CLI
    let args = build_cli().get_matches();

    // unpack various arguments from the CLI to use later on
    let limit = args.get_one::<usize>("limit");
    let textual = args.contains_id("textual");
    let template = args
        .get_one::<String>("template")
        .expect("template argument should be provided")
        .to_owned();

    // limit counter
    let mut counter = 0;

    // lock stdout for writing
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // construct a new generator to pull back from
    for document in Generator::from_path(template)? {
        // attempt to compact JSON
        let output = (!textual)
            // if it's enabled
            .then(|| &document)
            // by parsing the created document using serde_json
            .and_then(|document| serde_json::from_str(document).ok())
            // and then converting it back to a byte vector (compacted)
            .and_then(|contents: Value| serde_json::to_vec(&contents).ok())
            // and using the input bytes as a default
            .unwrap_or_else(|| document.into_bytes());

        // write entry to stdout
        stdout.write_all(&output)?;
        stdout.write_all(b"\n")?;

        // keep track
        counter += 1;

        // check the limit before next
        if let Some(limit) = limit {
            if &counter >= limit {
                break;
            }
        }
    }

    // never
    Ok(())
}

/// Creates a parser to deal with all CLI interaction.
///
/// All command line usage information can be found in the definitions
/// below, and follows the API of the `clap` library.
fn build_cli<'a>() -> Command {
    Command::new("")
        // package metadata from cargo
        .name(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        // add the template arguments
        .args(&[
            // limit: -l, --limit [1]
            Arg::new("limit")
                .help("An upper limit of documents to generate")
                .short('l')
                .long("limit")
                .num_args(1),
            // textual: -t, --textual
            Arg::new("textual")
                .help("Treat the input as textual, without JSON detection")
                .short('t')
                .long("textual"),
            // template: +required
            Arg::new("template")
                .help("Template to control generation format")
                .required(true),
        ])
        // settings required for parsing
        .arg_required_else_help(true)
        .hide_possible_values(true)
}
