//! This crate implement `liberty` data structre in Rust.
//!
//! Demo:
//! ``` rust
//! use liberty_db::{DefaultCtx, Library};
//! use std::{
//!   fs::File,
//!   io::{BufWriter, Write},
//! };
//! static TEMPLATE: &str = r#"
//! library(demo) {
//!   time_unit : "1ps";
//!   voltage_unit : "10mV";
//!   current_unit : "1uA";
//!   operating_conditions ( typical ) {
//!       process : 1;
//!       voltage : 1.1;
//!   }
//!   lu_table_template(delay_template_4x5) {
//!     variable_1 : total_output_net_capacitance;
//!     variable_2 : input_net_transition;
//!     index_1 ("1000.0, 1001.0, 1002.0, 1003.0");
//!     index_2 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0");
//!   }
//!   cell (DFF) {
//!     pin (D) {}
//!     pin (CK) {}
//!     pin (Q) {}
//!   }
//! }"#;
//! let mut library = Library::<DefaultCtx>::parse_lib(TEMPLATE).unwrap();
//! // modify library
//! library.cell.get_mut("DFF").map(|cell_dff| {
//!   cell_dff
//!     .pin
//!     .get_mut("CK".into())
//!     .map(|pin_ck| pin_ck.clock = Some(true))
//! });
//! // print library
//! println!("{library}");
//! // write library
//! let out_file = File::create("demo.lib").unwrap();
//! let mut writer = BufWriter::new(out_file);
//! write!(&mut writer, "{}", library).unwrap();
//! ```
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
    clippy::arbitrary_source_item_ordering,
    clippy::allow_attributes_without_reason,
    clippy::default_numeric_fallback,
    clippy::pattern_type_mismatch, // TODO: 
    clippy::too_long_first_doc_paragraph,
    clippy::partial_pub_fields,
    clippy::single_char_lifetime_names,
    clippy::while_let_on_iterator,
    clippy::iter_over_hash_type,
    clippy::separated_literal_suffix,
    clippy::single_call_fn,
    clippy::pub_use,
    clippy::or_fun_call,
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
    clippy::trivially_copy_pass_by_ref,
    clippy::default_numeric_fallback,
    clippy::unreadable_literal,
    clippy::type_complexity,
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::too_many_lines,
    clippy::non_ascii_literal,
    clippy::must_use_candidate,
    clippy::needless_raw_strings,
    clippy::cast_possible_truncation,
    clippy::as_conversions,
    clippy::needless_raw_string_hashes,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::field_reassign_with_default,
    clippy::undocumented_unsafe_blocks,
    clippy::or_fun_call,
  )
)]
pub use biodivine_lib_bdd;
pub use strum::IntoEnumIterator;
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
// pub mod str;
// pub use str::LibertyStr;
/// `pin` group structure.
pub mod pin;
pub use pin::Pin;
/// `timing` group structure.
pub mod timing;
pub use timing::Timing;
#[cfg(feature = "py")]
extern crate alloc;
#[cfg(feature = "py")]
mod py;
pub mod units;

pub mod ast;
pub use ast::Group;

/// CCSN relative attributes
pub mod ccsn;
mod ctx;
pub use ctx::{Ctx, DefaultCtx};
mod types;

#[test]
fn demo() {
  use crate::{DefaultCtx, Library};
  use std::{
    fs::File,
    io::{BufWriter, Write as _},
  };
  static TEMPLATE: &str = r#"
library(demo) {
  time_unit : "1ps";
  voltage_unit : "10mV";
  current_unit : "1uA";
  operating_conditions ( typical ) {
      process : 1;
      voltage : 1.1;
  }
  lu_table_template(delay_template_4x5) {
    variable_1 : total_output_net_capacitance;
    variable_2 : input_net_transition;
    index_1 ("1000.0, 1001.0, 1002.0, 1003.0");
    index_2 ("1000.0, 1001.0, 1002.0, 1003.0, 1004.0");
  }
  cell (DFF) {
    pin (D) {}
    pin (CK) {}
    pin (Q) {}
  }
}"#;
  let mut library = Library::<DefaultCtx>::parse_lib(TEMPLATE).unwrap();
  // modify library
  library.cell.get_mut("DFF").map(|cell_dff| {
    cell_dff
      .pin
      .get_mut("CK".into())
      .map(|pin_ck| pin_ck.clock = Some(true))
  });
  // print library
  println!("{library}");
  // write library
  let out_file = File::create("demo.lib").unwrap();
  let mut writer = BufWriter::new(out_file);
  write!(&mut writer, "{library}").unwrap();
}
