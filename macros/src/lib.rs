use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod name_idx;
mod group;
/// Auto create struct `GroupNameIdx` and implement 
/// `HashedGroup` for `GroupNameIdx` if the name(title) of 
/// group is the only hash-identity for that group.
/// ```
/// #[derive(liberty_macros::NameIdx)]
/// #[derive(liberty_macros::GroupHashed)]
/// pub struct Cell {
///   #[idx_len(1)]
///   _idx: Box<<Self as HashedGroup>::Idx>,
/// }
/// ```
/// Here are the options for `_idx`:
/// ```
/// #[idx_len(1)] -> name: String
/// #[idx_len(usize)] -> name: [String;usize]
/// #[idx_len(anyelse)] / not-define -> name: Vec[String]
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

/// Auto implement `GroupAttri`, with behavier of `Set=Vec<Self>`
/// 
/// Here are the options for each attributes:
/// + `#[arrti_type(simple)]`: `field: Type` or `field: Option<Type>`, 
/// where `Type` have implement `SimpleAttri`
/// + `#[arrti_type(complex)]`: `field: Type`, 
/// where `Type` have implement `ComplexAttri`
/// + `#[arrti_type(group)]`: `field: <Type as crate::ast::GroupAttri>::Set`,
/// where `Type` have implement `GroupAttri`
/// + `#[arrti_type(group_hashed)]`: `field: <Type as crate::ast::GroupAttri>::Set`,
/// where `Type` have implement `GroupAttri` and `HashedGroup`
/// 
/// Demo
/// ```
/// #[derive(liberty_macros::NameIdx)]
/// #[derive(liberty_macros::GroupHashed)]
/// pub struct Cell {
///   #[idx_len(1)]
///   _idx: Box<<Self as HashedGroup>::Idx>,
///   _undefined: crate::ast::UndefinedAttributes,
///   #[arrti_type(simple)]
///   area: Option<f64>,
///   #[arrti_type(group_hashed)]
///   ff: <Ff as crate::ast::GroupAttri>::Set,
///   #[arrti_type(group_hashed)]
///   pin: <Pin as crate::ast::GroupAttri>::Set,
/// }
/// ```
#[proc_macro_derive(Group, attributes(arrti_type))]
pub fn macro_group(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group::inner(&ast, false)
    .unwrap_or_else(|err| 
      err.to_compile_error().into()
    );
  toks.into()
}
/// Auto implement `GroupAttri`, with behavier of `Set=HashMap<Self::Idx,Self>`
/// 
/// Here are the options for each attributes:
/// + `#[arrti_type(simple)]`: `field: Type` or `field: Option<Type>`, 
/// where `Type` have implement `SimpleAttri`
/// + `#[arrti_type(complex)]`: `field: Type`, 
/// where `Type` have implement `ComplexAttri`
/// + `#[arrti_type(group)]`: `field: <Type as crate::ast::GroupAttri>::Set`,
/// where `Type` have implement `GroupAttri`
/// + `#[arrti_type(group_hashed)]`: `field: <Type as crate::ast::GroupAttri>::Set`,
/// where `Type` have implement `GroupAttri` and `HashedGroup`
/// 
/// Demo
/// ```
/// #[derive(liberty_macros::NameIdx)]
/// #[derive(liberty_macros::GroupHashed)]
/// pub struct Cell {
///   #[idx_len(1)]
///   _idx: Box<<Self as HashedGroup>::Idx>,
///   _undefined: crate::ast::UndefinedAttributes,
///   #[arrti_type(simple)]
///   area: Option<f64>,
///   #[arrti_type(group_hashed)]
///   ff: <Ff as crate::ast::GroupAttri>::Set,
///   #[arrti_type(group_hashed)]
///   pin: <Pin as crate::ast::GroupAttri>::Set,
/// }
/// ```
#[proc_macro_derive(GroupHashed, attributes(arrti_type))]
pub fn macro_group_hashed(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group::inner(&ast, true)
    .unwrap_or_else(|err| 
      err.to_compile_error().into()
    );
  toks.into()
}
