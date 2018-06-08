//! Nginx Config Parser (unofficial)
//! ================================
//!
//! This library contains parser and formatter of nginx config format
//! as well as AST types and visitors.
//!
//! [Docs](https://docs.rs/nginx-config/) |
//! [Github](https://github.com/tailhook/nginx-config/) |
//! [Crate](https://crates.io/crates/nginx-config)
//!
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]

extern crate combine;
#[cfg(feature="fuzzy_errors")] extern crate strsim;
#[macro_use] extern crate failure;
#[macro_use] extern crate matches;
#[cfg(test)] #[macro_use] extern crate pretty_assertions;

pub mod ast;
mod display;
mod error;
mod format;
mod grammar;
mod helpers;
mod position;
mod tokenizer;
mod value;
pub mod visitors;

mod core;
mod gzip;
mod headers;
mod proxy;
mod rewrite;

pub use grammar::{parse_main, parse_directives};
pub use format::Style;
pub use position::Pos;
pub use error::ParseError;
