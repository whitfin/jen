//! Generation of random data in a lazy pattern.
//!
//! This module offers the `Generator` struct, which can be used to
//! generate random data to place into a template. It implements the
//! `Iterator` trait, so you can easily chain and lazily generate
//! the amount of data you need.
use failure::Error;
use tera::{Context, Result as TResult, Tera};

use std::path::Path;

use crate::error;
use crate::helper;
use crate::helper::{BoxedHelper, BoxedHelpers};

/// Simple random (templated) data generator.
///
/// This struct implements `Iterator` to enable easy generation of
/// random data in a lazy fashion. To achieve this templates are
/// validated at struct creation time to guarantee correctness in
/// the `Iterator` chain to avoid `Result` types.
///
/// Custom helpers can be provided alongside this structure if you
/// wish to add your own helpers in addition to defaults, or maybe
/// to override them entirely.
pub struct Generator {
    ctx: Context,
    tera: Tera,
}

impl Generator {
    /// Loads a new `Generator` from a template path.
    pub fn from_path<S: AsRef<Path>>(source: S) -> Result<Self, Error> {
        Self::from_path_with_helpers(source, helper::builtin())
    }

    /// Loads a new `Generator` from a template path and set of helpers.
    pub fn from_path_with_helpers<S>(source: S, helpers: BoxedHelpers) -> Result<Self, Error>
    where
        S: AsRef<Path>,
    {
        init(helpers, |tera| {
            tera.add_template_file(source, Some("template"))
        })
    }

    /// Initializes a new `Generator` from a template string.
    pub fn from_string<S: AsRef<str>>(source: S) -> Result<Self, Error> {
        Self::from_string_with_helpers(source, helper::builtin())
    }

    /// Initializes a new `Generator` from a template string and set of helpers.
    pub fn from_string_with_helpers<S>(source: S, helpers: BoxedHelpers) -> Result<Self, Error>
    where
        S: AsRef<str>,
    {
        init(helpers, |tera| {
            tera.add_raw_template("template", source.as_ref())
        })
    }

    /// Creates a new `String` from the internal template.
    pub fn create(&mut self) -> String {
        self.tera
            .render("template", &self.ctx)
            .expect("templates are validated at creation time")
    }
}

// Implementation of `Iterator`.
impl Iterator for Generator {
    type Item = String;

    /// Fetches a new `String` of random data.
    fn next(&mut self) -> Option<String> {
        Some(self.create())
    }
}

/// Initialzes a new `Generator` with a custom registration function provided.
fn init<R>(helpers: Vec<(&str, BoxedHelper)>, register: R) -> Result<Generator, Error>
where
    R: FnOnce(&mut Tera) -> TResult<()>,
{
    // construct base values
    let ctx = Context::new();
    let mut tera = Tera::default();

    // attach all helpers as functions
    for (name, addon) in helpers {
        tera.register_function(name, addon);
    }

    // load the template into the Tera context
    register(&mut tera).map_err(error::raw)?;

    // test run to pre-validate the templates
    if tera.render("template", &ctx).is_err() {
        return Err(error::raw("Unable to parse source template"));
    }
    // able to create generator
    Ok(Generator { ctx, tera })
}

/// Deprecated implementations.
impl Generator {
    /// Constructs a new `Generator` from a template path.
    #[deprecated(note = "Please migrate to from_path for clarity")]
    pub fn new<S: AsRef<Path>>(source: S) -> Result<Self, Error> {
        Self::from_path(source)
    }

    /// Constructs a new `Generator` from a template path and set of helpers.
    #[deprecated(note = "Please migrate to from_path_with_helpers for clarity")]
    pub fn new_with_helpers<S>(source: S, helpers: Vec<(&str, BoxedHelper)>) -> Result<Self, Error>
    where
        S: AsRef<Path>,
    {
        Self::from_path_with_helpers(source, helpers)
    }
}
