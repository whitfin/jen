//! Parent module for helper function exports.
//!
//! This module contains functions to construct helpers which can
//! be baked into the `Generator` to control additional features
//! against the internal templating engine.
use std::collections::HashMap;
use tera::{Result, Value};

mod custom;
mod fake;

/// Public type for a boxed helper definition for `Function` conversion.
pub type BoxedHelper = Box<dyn Fn(&HashMap<String, Value>) -> Result<Value> + Sync + Send>;

/// Public type for helpers during `Generator` creation.
pub type BoxedHelpers<'a> = Vec<(&'a str, BoxedHelper)>;

/// Returns a named vector containing named helper pairs.
///
/// This function returns all built-in helpers, so you can append
/// your own helpers as necessary. In the case of a name clash with
/// a built-in helper, the behaviour should be deemed undefined.
pub fn builtin() -> Vec<(&'static str, BoxedHelper)> {
    let mut vec = custom::helpers();
    vec.append(&mut fake::helpers());
    vec
}
