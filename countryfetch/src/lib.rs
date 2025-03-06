#![feature(let_chains)]
#![allow(clippy::multiple_crate_versions, reason = "todo")]
#![allow(clippy::cargo_common_metadata, reason = "later")]

pub mod args;
pub mod cache;
pub mod country;
pub mod country_format;
pub mod location;

pub use args::Args;
pub use country::Country;
pub use location::Location;
