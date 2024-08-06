//! This crate implement `liberty` data structre in Rust.
//! <script>
//! IFRAME('https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html');
//! </script>

#![doc(
    // The following are document setting according to
    // https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html
    // For html-pdf-viewer setting:
    // https://tinytip.co/tips/html-pdf-params/
    // html_favicon_url = "https://example.com/favicon.ico",
    // html_logo_url = "https://example.com/logo.jpg",
    html_playground_url = "https://play.rust-lang.org",
)]
// #![cfg_attr(
//     feature = "nightly",
//     feature(
//         test,
//         core_intrinsics,
//         dropck_eyepatch,
//         min_specialization,
//         extend_one,
//         allocator_api,
//         slice_ptr_get,
//         nonnull_slice_from_raw_parts,
//         maybe_uninit_array_assume_init,
//         build_hasher_simple_hash_one
//     )
// )]
#![deny(
    // The following are allowed by default lints according to
    // https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html
    anonymous_parameters,
    bare_trait_objects,
    // box_pointers,
    elided_lifetimes_in_paths, // allow anonymous lifetime
    missing_debug_implementations,
    // missing_docs, // TODO: add documents
    // trivial_casts, // TODO: remove trivial casts in code
    trivial_numeric_casts,
    // unsafe_code,
    unstable_features,
    unused_extern_crates,
    unused_import_braces,
    unreachable_pub, // allow clippy::redundant_pub_crate lint instead
    // unused_qualifications,
    unused_results, // TODO: fix unused results
    variant_size_differences,
    // warnings, // treat all wanings as errors
    clippy::all,
    clippy::restriction,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo
)]
#![allow(
    // Some explicitly allowed Clippy lints, must have clear reason to allow
    clippy::blanket_clippy_restriction_lints, // allow clippy::restriction
    clippy::implicit_return, // actually omitting the return keyword is idiomatic Rust code
    clippy::module_name_repetitions, // repeation of module name in a struct name is not big deal
    clippy::multiple_crate_versions, // multi-version dependency crates is not able to fix
    clippy::panic, // allow debug_assert, panic in production code
    clippy::panic_in_result_fn,
    clippy::missing_errors_doc, // TODO: add error docs
    clippy::exhaustive_structs,
    clippy::exhaustive_enums,
    clippy::missing_panics_doc, // TODO: add panic docs
    clippy::panic_in_result_fn,
    clippy::print_stdout,
    clippy::use_debug
)]
#![warn(
    clippy::todo,
    dead_code,
    missing_copy_implementations, // Copy may cause unnecessary memory copy
    missing_docs,
    // single_use_lifetimes, // TODO: fix lifetime names only used once
    unused_qualifications,
)]

#[macro_use]
extern crate log;
pub use arcstr;
pub use arcstr::ArcStr;
pub use mut_set::MutSet as GroupSet;
pub use ordered_float::NotNan;
/// `bundle` group structure.
pub mod bundle;
/// `bus` group structure.
pub mod bus;
/// `cell` group structure.
pub mod cell;
pub use cell::Cell;
/// Common items/miscs.
pub mod common;
/// `Boolean Expression`, `SDF Expression`, and so on.
pub mod expression;
/// `internal_power` group structure.
pub mod internal_power;
/// `Library` group structure, top level of liberty format.
pub mod library;
pub use library::Library;
/// `pin` group structure.
pub mod pin;
pub use pin::Pin;
/// `timing` group structure.
pub mod timing;
/// Partially re-exported [uom](https://crates.io/crates/uom) quantities and measurement units
/// used in the library public interface.
pub mod units;

pub mod ast;

/// CCSN relative attributes
pub mod ccsn;
mod types;
mod util;
