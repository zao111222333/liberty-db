use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

mod group;

/// Auto implement `GroupAttri` for `struct`, which should have a field named
/// `_undefined: crate::ast::AttributeList` to store undefined attribute.
/// 
/// when that `Struct` have a field named `_id` (`_id: <Self as HashedGroup>::Id`),
/// it will implement additional function to handle group index. What's more, when
/// there are `#[id_len(1)]` on the `_id`, it will implement `HashedGroup`.
/// + `#[id_len(1)]`: `<Self as HashedGroup>::Id=String`
/// + `#[id_len(-1)]`: `<Self as HashedGroup>::Id=Vec<String>`
/// + `#[id_len(n)]`, where `n` is some usize and `n` > 1: `<Self as HashedGroup>::Id=[String;n]`
/// 
/// Here are the options for each attributes:
/// + `#[arrti_type(simple)]`: `field: Type` or `field: Option<Type>`, 
/// where `Type` have implement `SimpleAttri`
/// + `#[arrti_type(complex)]`: `field: Type`, 
/// where `Type` have implement `ComplexAttri`
/// + `#[arrti_type(group)]`, Type need implement `GroupAttri`
///   + when field is `Option<Type>`
///   + when field is `Vec<Type>`
///   + when field is `HashSet<Type>`
/// 
/// Demo
/// ```
/// use crate::ast::{AttributeList,HashedGroup};
/// #[derive(Default,Debug)]
/// #[derive(liberty_macros::Group)]
/// struct Timing{
///   _undefined: AttributeList,
///   #[arrti_type(simple)]
///   timing_type: Option<TimingType>,
/// }
/// #[derive(liberty_macros::NameIdx)]
/// #[derive(liberty_macros::Group)]
/// pub struct Cell {
///   #[id_len(1)]
///   _id: <Self as HashedGroup>::Id,
///   _undefined: AttributeList,
///   #[arrti_type(simple)]
///   area: Option<f64>,
///   #[arrti_type(group)]
///   ff: HashMap<<Ff as HashedGroup>::Id,Ff>,
///   #[arrti_type(group)]
///   pin: HashMap<<Pin as HashedGroup>::Id,Pin>,
/// }
/// ```
#[proc_macro_derive(Group, attributes(arrti_type,id_len))]
pub fn macro_group(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group::inner(&ast)
    .unwrap_or_else(|err| 
      err.to_compile_error().into()
    );
  toks.into()
}