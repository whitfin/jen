//! Jen is a utility to generate JSON documents from templates.
//!
//! It provides a very easy and quick way to generate test data using
//! a simple command line API. Jen was written because there are few
//! good tools for this purpose which work in a terminal. Most are
//! available via a browser, which severely limits the amount of data
//! you can feasibly generate.
//!
//! The aim of this tool is to be sufficiently fast, whilst providing
//! various options to make generation more convenient. It was written
//! to scratch a personal itch, but there's no reason new features can
//! not be added if requested.
//!
//! Most of the underlying tooling is provided via the `Tera` crate
//! for templating, and the `fake` crate for data construction. Jen
//! itself is simply a binding around these two crates to provide a
//! convenience bridge between them. Go check them out!
#![doc(html_root_url = "https://docs.rs/jen/1.0.0")]
use clap::{value_t, App, AppSettings, Arg};
use serde::Serialize;
use serde_json::Value;

mod helper;

mod generator;
use generator::Generator;

mod error;
use error::Error;

/// Entry point of Jen.
fn main() {
    // execute and log errors
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

/// Executes the main tooling of Jen.
fn run() -> Result<(), Error> {
    // parse the arguments from the CLI
    let args = build_cli().get_matches();

    // unpack various arguments from the CLI to use later on
    let amount = value_t!(args, "amount", usize).unwrap_or_else(|_| 1);
    let combine = args.is_present("combine");
    let prettied = args.is_present("pretty");
    let template = args
        .value_of("template")
        .expect("template argument should be provided");

    // construct a new templating instance
    let generator = Generator::new(template)?;
    let mut buffer = Vec::with_capacity(amount);

    // fetch some amount of generated data
    for created in generator.take(amount) {
        // parse them into JSON, due to the buffer
        let parsed = serde_json::from_str::<Value>(&created)?;

        // append buffer
        if combine {
            buffer.push(parsed);
        } else {
            print(&parsed, prettied)?;
        }
    }

    // print buffer
    if combine {
        print(&buffer, prettied)?;
    }

    // done!
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
            // amount: -a, --amount [1]
            Arg::with_name("amount")
                .help("The amount of documents to generate in this batch")
                .short("a")
                .long("amount")
                .takes_value(true)
                .default_value("1"),
            // combine: -c, --combine
            Arg::with_name("combine")
                .help("Whether to combine documents into a JSON array")
                .short("c")
                .long("combine"),
            // amount: -p, --pretty
            Arg::with_name("pretty")
                .help("Whether to pretty print the output documents")
                .short("p")
                .long("pretty"),
            // template: +required
            Arg::with_name("template")
                .help("Template to control JSON generation")
                .required(true),
        ])
        // settings required for parsing
        .settings(&[
            AppSettings::ArgRequiredElseHelp,
            AppSettings::HidePossibleValuesInHelp,
        ])
}

/// Prints a value to stdout, making the output pretty when configured.
fn print<S: Serialize>(value: &S, prettied: bool) -> Result<(), Error> {
    // formatting for pretty
    let output = if prettied {
        serde_json::to_string_pretty(value)?
    } else {
        serde_json::to_string(value)?
    };

    // write to stdout
    println!("{}", output);

    // done
    Ok(())
}
