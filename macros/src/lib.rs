mod attribute;
mod duplicate;
mod group;

use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Group, attributes(liberty))]
pub fn macro_group(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group::inner(&ast).unwrap_or_else(|err| err.to_compile_error());
  toks.into()
}

#[proc_macro_derive(Nothing, attributes(liberty))]
pub fn macro_nothing(_: TokenStream) -> TokenStream {
  quote::quote!().into()
}

#[proc_macro_derive(Duplicate, attributes(duplicated))]
pub fn macro_duplicate(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = duplicate::inner(ast).unwrap_or_else(|err| err.to_compile_error());
  toks.into()
}
