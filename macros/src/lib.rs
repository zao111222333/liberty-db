use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod attribute;
mod enum_token;
mod group;
mod macro_match;
mod trie_tree;

/// ```
/// // UndefinedAttribute
/// #[liberty(undefined)]
/// // GroupComments
/// #[liberty(comments)]
/// // Auto vector Id: Vec<ArcStr>
/// #[liberty(id(title=0))]
/// // Auto slice Id: [ArcStr:2]
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
#[proc_macro_derive(Group, attributes(liberty, default))]
pub fn macro_group(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group::inner(&ast).unwrap_or_else(|err| err.to_compile_error());
  toks.into()
}

#[proc_macro_derive(Nothing, attributes(liberty, default))]
pub fn macro_nothing(_: TokenStream) -> TokenStream {
  quote::quote!().into()
}

#[proc_macro_derive(EnumToken, attributes(token))]
pub fn macro_enum(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = enum_token::inner(&ast).unwrap_or_else(|err| err.to_compile_error());
  toks.into()
}

#[proc_macro_attribute]
pub fn macro_match(attr: TokenStream, item: TokenStream) -> TokenStream {
  let args = parse_macro_input!(attr as macro_match::MacroArgs);
  let input = parse_macro_input!(item as DeriveInput);
  let toks = macro_match::inner(args, input).unwrap_or_else(|err| err.to_compile_error());
  toks.into()
}
