//! Error definitions and utility functions.
//!
//! The enclosing types should be used rather than the underlying type
//! to allow us to easily change error definitions as needed. All errors
//! should be created by utilities in this module when required.
use std::fmt::Display;

/// Error type to encompass application errors.
///
/// This type should be used directly rather than relying on the actual
/// type definition to enable easily changing definitions under the hood.
pub type Error = failure::Error;

/// Constructs a raw `Error` value.
///
/// This creates an `Error` using the `Display` trait to generate the
/// message for the error itself. This handles many CLI based cases.
#[inline]
pub(crate) fn raw<D: Display>(d: D) -> Error {
    failure::format_err!("{}", d.to_string())
}
