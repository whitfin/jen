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
#![doc(html_root_url = "https://docs.rs/jen/1.4.0")]
use clap::{value_t, App, AppSettings, Arg};
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
    let limit = value_t!(args, "limit", usize).ok();
    let textual = args.is_present("textual");
    let template = args
        .value_of("template")
        .expect("template argument should be provided")
        .to_owned();

    // limit counter
    let mut counter = 0;

    // lock stdout for writing
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // construct a new generator to pull back from
    for document in Generator::from_path(template)? {
        // attempt to detect JSON values and compact them
        let output = (!textual)
            .then(|| &document)
            .and_then(|created| serde_json::from_str::<Value>(created).ok())
            .and_then(|parsed| serde_json::to_vec(&parsed).ok())
            .unwrap_or_else(|| document.into_bytes());

        // write entry to stdout
        stdout.write_all(&output)?;
        stdout.write_all(b"\n")?;

        // keep track
        counter += 1;

        // check the limit before next
        if let Some(limit) = limit {
            if counter >= limit {
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
fn build_cli<'a, 'b>() -> App<'a, 'b> {
    App::new("")
        // package metadata from cargo
        .name(env!("CARGO_PKG_NAME"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .version(env!("CARGO_PKG_VERSION"))
        // add the template arguments
        .args(&[
            // limit: -l, --limit [1]
            Arg::with_name("limit")
                .help("An upper limit of documents to generate")
                .short("l")
                .long("limit")
                .takes_value(true),
            // textual: -t, --textual
            Arg::with_name("textual")
                .help("Treat the input as textual, without JSON detection")
                .short("t")
                .long("textual"),
            // template: +required
            Arg::with_name("template")
                .help("Template to control generation format")
                .required(true),
        ])
        // settings required for parsing
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::HidePossibleValuesInHelp,
        ])
}
