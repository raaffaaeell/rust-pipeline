#![warn(
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    clippy::cargo_common_metadata
)]
#![deny(
    future_incompatible,
    missing_debug_implementations,
    rust_2018_idioms,
    clippy::wildcard_dependencies
)]
#![forbid(unsafe_code)]

pub mod annotation;
pub mod cas;
pub mod engine;
pub mod error;
pub mod feature;
pub mod pipeline;
pub mod textengine;
