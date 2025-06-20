use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod attribute;
mod duplicate;
mod group;

/// ```
/// // UndefinedAttribute
/// #[liberty(undefined)]
/// // GroupComments
/// #[liberty(comments)]
/// // Auto vector Id: Vec<String>
/// #[liberty(id(title=0))]
/// // Auto slice Id: [String:2]
/// #[liberty(id(title=2))]
/// // GroupId
/// #[liberty(id)]
/// // Simple liberty attribute, defualt=Default
/// #[liberty(simple)]
/// // Simple liberty attribute, Default
/// #[liberty(simple(type=Default))]
/// // Simple liberty attribute, Option
/// #[liberty(simple(type = Option))]
/// // Complex liberty attribute, default=Default
/// #[liberty(complex)]
/// // Complex liberty attribute, Default
/// #[liberty(complex(type=Default))]
/// // Complex liberty attribute, Option
/// #[liberty(complex(type = Option))]
/// // Complex group attribute, defualt=Option
/// #[liberty(group)]
/// // Complex group attribute, Option
/// #[liberty(group(type = Option))]
/// // Complex group attribute, Map
/// #[liberty(group(type=Map))]
/// // Complex group attribute, Vec
/// #[liberty(group(type=Vec))]
/// ```
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
