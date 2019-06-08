#![doc(html_root_url = "https://docs.rs/jen/1.0.0")]
use clap::{value_t, App, AppSettings, Arg};
use serde_json::Value;
use tera::{Context, Tera};

mod addons;

mod errors;
use errors::Error;

fn main() {
    // execute and log errors
    if let Err(e) = run() {
        eprintln!("{}", e);
    }
}

fn run() -> Result<(), Error> {
    let args = build_cli().get_matches();

    let amount = value_t!(args, "amount", usize).unwrap_or_else(|_| 1);

    let template = args
        .value_of("template")
        .expect("template argument should be provided");

    let mut tera = Tera::default();

    for (name, addon) in addons::all() {
        tera.register_function(name, addon);
    }

    tera.add_template_file(template, Some("template"))
        .map_err(errors::raw)?;

    let combine = args.is_present("combine");
    let prettied = args.is_present("pretty");
    let mut buffer = Vec::new();

    for _ in 0..amount {
        let rendered = tera
            .render("template", &Context::new())
            .map_err(errors::raw)?;

        let parsed = serde_json::from_str::<Value>(&rendered)?;

        if combine {
            buffer.push(parsed);
            continue;
        }

        let output = if prettied {
            serde_json::to_string_pretty(&parsed)?
        } else {
            serde_json::to_string(&parsed)?
        };

        println!("{}", output);
    }

    if combine {
        let output = if prettied {
            serde_json::to_string_pretty(&buffer)?
        } else {
            serde_json::to_string(&buffer)?
        };

        println!("{}", output);
    }

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
