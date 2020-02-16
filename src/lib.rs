#![deny(
    clippy::all,
    warnings,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused,
    macro_use_extern_crate,
// TODO    missing_docs,
    non_ascii_idents,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    unreachable_pub,
    unsafe_code,
    unused_import_braces,
    unused_lifetimes,
    unused_qualifications
)]

pub mod client;
pub mod device;
pub mod episode;
pub mod error;
pub mod subscription;
pub mod suggestion;
