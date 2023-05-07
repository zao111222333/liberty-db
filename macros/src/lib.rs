use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod name_idx;
mod group;
/// Auto create struct `GroupNameIdx` and implement 
/// `HashedGroup` for `GroupNameIdx` if the name(title) of 
/// group is the only hash-identity for that group.
/// ```
/// #[derive(liberty_macros::NameIdx)]
/// #[derive(liberty_macros::Group)]
/// pub struct Cell {
///   #[idx_len(1)]
///   _idx: Box<<Self as HashedGroup>::Idx>,
/// }
/// ```
/// Here are the options for `_idx`:
/// + `#[idx_len(1)]` -> name: String
/// + `#[idx_len(usize)]` -> name: [String;usize]
/// + `#[idx_len(anyelse)]` or `not-define` -> name: Vec[String]
#[proc_macro_derive(NameIdx,attributes(idx_len))]
pub fn macro_name_idx(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = name_idx::inner(&ast)
    .unwrap_or_else(|err| 
      err.to_compile_error().into()
    );
  toks.into()
}

/// Auto implement `GroupAttri`, when that `Struct` have a field named `_idx` (`_idx: Box<<Self as HashedGroup>::Idx>`),
/// it will implement additional function to handle group index.
/// 
/// Here are the options for each attributes:
/// + `#[arrti_type(simple)]`: `field: Type` or `field: Option<Type>`, 
/// where `Type` have implement `SimpleAttri`
/// + `#[arrti_type(complex)]`: `field: Type`, 
/// where `Type` have implement `ComplexAttri`
/// + `#[arrti_type(group)]`, Type need implement `GroupAttri`
///   + when field is `Option<Type>`
///   + when field is `Vec<Type>`
///   + when field is `HashMap<<Type as HashedGroup>::Idx,Type>`
/// 
/// Demo
/// ```
/// use crate::ast::{UndefinedAttributes,HashedGroup};
/// #[derive(Default,Debug)]
/// #[derive(liberty_macros::Group)]
/// struct Timing{
///   _undefined: UndefinedAttributes,
///   #[arrti_type(simple)]
///   timing_type: Option<TimingType>,
/// }
/// #[derive(liberty_macros::NameIdx)]
/// #[derive(liberty_macros::Group)]
/// pub struct Cell {
///   #[idx_len(1)]
///   _idx: Box<<Self as HashedGroup>::Idx>,
///   _undefined: UndefinedAttributes,
///   #[arrti_type(simple)]
///   area: Option<f64>,
///   #[arrti_type(group)]
///   ff: HashMap<<Ff as HashedGroup>::Idx,Ff>,
///   #[arrti_type(group)]
///   pin: HashMap<<Pin as HashedGroup>::Idx,Pin>,
/// }
/// ```
#[proc_macro_derive(Group, attributes(arrti_type))]
pub fn macro_group(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group::inner(&ast)
    .unwrap_or_else(|err| 
      err.to_compile_error().into()
    );
  toks.into()
}