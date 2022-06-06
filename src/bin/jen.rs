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
#![doc(html_root_url = "https://docs.rs/jen/1.3.1")]
use clap::{value_t, App, AppSettings, Arg};
use serde_json::Value;

use jen::error::Error;
use jen::generator::Generator;

use std::io::{self, Write};
use std::sync::{Arc, RwLock};
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
    let cpus = num_cpus::get();
    let core = cpus.to_string();
    let args = build_cli(&core).get_matches();

    // unpack various arguments from the CLI to use later on
    let limit = value_t!(args, "limit", usize).ok();
    let threads = value_t!(args, "workers", usize).unwrap_or_else(|_| cpus);
    let textual = args.is_present("textual");
    let template = args
        .value_of("template")
        .expect("template argument should be provided")
        .to_owned();

    // create a tracker used to keep counts
    let tracker = Arc::new(RwLock::new(0));

    // create all workers across multiple threads
    let workers = (0..threads).map(move |_| {
        // clone the state for ownership
        let tracker = tracker.clone();
        let template = template.clone();

        // spawn a thread to generate a batch
        thread::spawn(move || -> Result<(), Error> {
            // construct a new generator on the thread
            let mut generator = Generator::from_path(template)?;

            // check we don't over do it
            if let Some(limit) = limit {
                if *tracker.read().unwrap() >= limit {
                    return Ok(());
                }
            }

            loop {
                // fetch some amount of generated data from the generator
                let created = generator.next().expect("unable to generate");

                {
                    // lock the tracker for the time being
                    let mut counter = tracker.write().unwrap();

                    // check the limit before we write
                    if let Some(limit) = limit {
                        if *counter >= limit {
                            return Ok(());
                        }
                    }

                    // increment the counter
                    *counter += 1;
                }

                // no extras
                if textual {
                    println!("{}", created);
                    continue;
                }

                // compact the JSON to reduce any template based whitespace
                let parsed = serde_json::from_str::<Value>(&created)?;
                let output = serde_json::to_vec(&parsed)?;

                // lock stdout for writing
                let stdout = io::stdout();
                let mut stdout = stdout.lock();

                // write to stdout
                stdout.write_all(&output)?;
                stdout.write_all(b"\n")?;
            }
        })
    });

    // join all worker threads
    for worker in workers {
        worker.join().unwrap()?;
    }

    // done!
    Ok(())
}

/// Creates a parser to deal with all CLI interaction.
///
/// All command line usage information can be found in the definitions
/// below, and follows the API of the `clap` library.
fn build_cli<'a, 'b>(cpus: &'a str) -> App<'a, 'b> {
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
                .help("Treat the input as textual, rather than JSON")
                .short("t")
                .long("textual"),
            // workers: -w, --workers [4]
            Arg::with_name("workers")
                .help("Number of threads used to generate data")
                .short("w")
                .long("workers")
                .takes_value(true)
                .default_value(cpus),
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
