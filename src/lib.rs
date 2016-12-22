//! #Humanize
//! A library to help you interact with users in a human-friendly way.
//!
//! ## What it does
//! Humanize contains the following modules:
//!
//! * `Bool`, to parse Boolean values from user input, with several languages supported
//! * `Numbers`, which provides common representations of numerical values
//! * `Strings`, to perform common manipulations on string types for user facing content
//!
//! For file size humanization, see the dedicated [humansize crate](http://www.crates.io/crates/humansize)
//!
//! To learn more, select the documentation for a specific module below, or check out the examples folder in the source.
//!
#[macro_use]
extern crate lazy_static;

pub mod bool;
pub mod numbers;
pub mod strings;

pub use numbers::HumanizeNumbers;
pub use bool::ToBool;
