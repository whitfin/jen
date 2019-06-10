//! Generation of random data in a lazy pattern.
//!
//! This module offers the `Generator` struct, which can be used to
//! generate random data to place into a template. It implements the
//! `Iterator` trait, so you can easily chain and lazily generate
//! the amount of data you need.
use failure::Error;
use tera::{Context, Tera};

use std::path::Path;

use crate::error;
use crate::helper::{self, Helper};

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
    /// Constructs a new `Generator` from a template path.
    pub fn new<P>(source: P) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        Self::new_with_helpers(source, helper::all())
    }

    /// Constructs a new `Generator` from a template path and set of helpers.
    pub fn new_with_helpers<P>(source: P, helpers: Vec<(&str, Helper)>) -> Result<Self, Error>
    where
        P: AsRef<Path>,
    {
        // construct base values
        let ctx = Context::new();
        let mut tera = Tera::default();

        // attach all helpers as functions
        for (name, addon) in helpers {
            tera.register_function(name, addon);
        }

        // load the template file into the Tera context
        tera.add_template_file(source, Some("template"))
            .map_err(error::raw)?;

        // test run to pre-validate the templates
        if tera.render("template", &ctx).is_err() {
            return Err(error::raw("Unable to parse source template"));
        }

        // able to create gen
        Ok(Self { ctx, tera })
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
