use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod name_idx;
mod group;
/// Auto create struct `GroupNameIdx` and implement 
/// `HashedGroup` for `GroupNameIdx` if the name(title) of 
/// goroup is the only hash-identity for that group.
/// ```
/// #[derive(liberty_macros::NameIdx)]
/// #[derive(liberty_macros::GroupHashed)]
/// pub struct Cell {
///   #[idx_len(1)]
///   _idx: Box<<Self as HashedGroup>::Idx>,
/// }
/// ```
/// ```
/// #[idx_len(1)] -> name: String
/// #[idx_len(usize)] -> name: [String;usize]
/// #[idx_len(anyelse)] -> name: Vec[String]
/// not-define idx_len -> name: Vec[String]
/// ```
#[proc_macro_derive(NameIdx,attributes(idx_len))]
pub fn macro_name_idx(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = name_idx::inner(&ast)
    .unwrap_or_else(|err| 
      err.to_compile_error().into()
    );
  toks.into()
}

#[proc_macro_derive(Group, attributes(arrti_type))]
pub fn macro_group(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group::inner(&ast, false)
    .unwrap_or_else(|err| 
      err.to_compile_error().into()
    );
  toks.into()
}

#[proc_macro_derive(GroupHashed, attributes(arrti_type))]
pub fn macro_group_hashed(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group::inner(&ast, true)
    .unwrap_or_else(|err| 
      err.to_compile_error().into()
    );
  toks.into()
}
