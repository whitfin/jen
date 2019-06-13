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
#![doc(html_root_url = "https://docs.rs/jen/1.0.0")]
use clap::{value_t, App, AppSettings, Arg};
use serde::Serialize;
use serde_json::Value;

use jen::error::Error;
use jen::generator::Generator;

use std::io::{self, Write};
use std::sync::{Arc, Mutex};
use std::thread;

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
    let threads = value_t!(args, "workers", usize).unwrap_or_else(|_| 4);
    let combine = args.is_present("combine");
    let textual = args.is_present("textual");
    let prettied = args.is_present("pretty");
    let template = args
        .value_of("template")
        .expect("template argument should be provided")
        .to_owned();

    // construct a new buffer instance
    let buffer = Vec::with_capacity(amount);
    let buffer = Mutex::new(buffer);
    let buffer = Arc::new(buffer);

    // calculate options for threading
    let total = amount / threads;
    let offset = amount % threads;

    // allocate vectors to store amount offsets
    let mut amounts = Vec::with_capacity(threads);

    // handle all remainders
    for _ in 0..offset {
        amounts.push(total + 1);
    }

    // push the total up to limit
    while amounts.len() < threads {
        amounts.push(total);
    }

    // spawn all workers
    let workers = amounts
        .into_iter()
        .map(|amount| {
            // clone the state for ownership
            let buffer = buffer.clone();
            let template = template.clone();

            // spawn a thread to generate a batch
            thread::spawn(move || -> Result<(), Error> {
                // construct a new generator on the thread
                let generator = Generator::new(template)?;

                // fetch some amount of generated data
                for created in generator.take(amount) {
                    // raw has no extras
                    if textual {
                        println!("{}", created);
                        continue;
                    }

                    // parse them into JSON, due to the buffer
                    let parsed = serde_json::from_str::<Value>(&created)?;

                    // append buffer
                    if combine {
                        buffer.lock().unwrap().push(parsed);
                    } else {
                        print(&parsed, prettied)?;
                    }
                }

                // done!
                Ok(())
            })
        })
        .collect::<Vec<_>>();

    // join all worker threads
    for worker in workers {
        worker.join().unwrap()?;
    }

    // print buffer
    if combine {
        let buffer = Arc::try_unwrap(buffer).expect("able to take Arc");
        let buffer = buffer.into_inner().expect("able to take Mutex");
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
                .help("Whether to combine documents into an array")
                .short("c")
                .long("combine"),
            // amount: -p, --pretty
            Arg::with_name("pretty")
                .help("Whether to pretty print the output documents")
                .short("p")
                .long("pretty"),
            // textual: -t, --textual
            Arg::with_name("textual")
                .help("Treat the input as textual, rather than JSON")
                .short("t")
                .long("textual"),
            // workers: -w, --workers [4]
            Arg::with_name("workers")
                .help("Number of threads used to generate data")
                .short("w")
                .long("workers")
                .takes_value(true)
                .default_value("4"),
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
        serde_json::to_vec_pretty(value)?
    } else {
        serde_json::to_vec(value)?
    };

    // lock stdout for writing
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    // write to stdout
    stdout.write_all(&output)?;
    stdout.write_all(b"\n")?;

    // done
    Ok(())
}
