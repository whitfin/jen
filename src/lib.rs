//! Jen is a utility to generate JSON documents from templates.
//!
//! This library exposes the core functionality of the CLI tooling to
//! enable it to be used programmatically. This allows developers to
//! construct custom data on the fly during unit tests, for example.
//!
//! The aim of this tool is to be reasonably fast, whilst being mostly
//! configurable to allow attaching your own helpers as needed. There
//! are many built-in helpers for generating data, which are used by
//! default.
//!
//! Most of the underlying tooling is provided via the `Tera` crate
//! for templating, and the `fake` crate for data construction. Jen
//! itself is simply a binding around these two crates to provide a
//! convenience bridge between them. Go check them out!
#![allow(clippy::unnecessary_wraps)]
#![doc(html_root_url = "https://docs.rs/jen/1.2.0")]
pub mod error;
pub mod generator;
pub mod helper;
