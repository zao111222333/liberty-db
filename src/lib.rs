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
    clippy::pattern_type_mismatch, // TODO: 
    clippy::partial_pub_fields,
    clippy::single_char_lifetime_names,
    clippy::while_let_on_iterator,
    clippy::iter_over_hash_type,
    clippy::separated_literal_suffix,
    clippy::single_call_fn,
    clippy::pub_use,
    clippy::pub_with_shorthand,
    clippy::ignored_unit_patterns,
    clippy::mod_module_files,
    clippy::unreachable,
    clippy::missing_trait_methods,
    clippy::min_ident_chars,
    clippy::missing_docs_in_private_items,
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
    clippy::absolute_paths,
    clippy::use_debug,
    clippy::question_mark_used,
    clippy::used_underscore_binding,
)]
#![warn(
    clippy::todo,
    clippy::manual_map,
    clippy::shadow_reuse,
    clippy::option_if_let_else,
    clippy::wildcard_enum_match_arm,
    clippy::needless_return,
    clippy::same_name_method,
    clippy::missing_inline_in_public_items,
    clippy::doc_markdown,
    dead_code,
    missing_copy_implementations, // Copy may cause unnecessary memory copy
    // missing_docs,
    // single_use_lifetimes, // TODO: fix lifetime names only used once
    unused_qualifications,
)]
#![cfg_attr(
  test,
  allow(
    dead_code,
    unused,
    unused_imports,
    unused_results,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::needless_raw_strings,
    clippy::cast_possible_truncation,
    clippy::as_conversions,
    clippy::needless_raw_string_hashes,
    clippy::arithmetic_side_effects,
  )
)]

pub use arcstr;
pub use arcstr::ArcStr;
pub use mut_set::MutSet as GroupSet;
pub use ordered_float::NotNan;
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
pub mod util;
