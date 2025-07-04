use core::fmt::Debug;
use std::collections::HashMap;

use proc_macro2::Ident;
use quote::{ToTokens, quote};
use syn::{
  Expr, Path, Token, Type,
  parse::{Parse, ParseStream},
};

#[derive(Debug, Clone, Copy)]
pub(crate) enum InternalType {
  /// `name`
  Name { flatten: bool },
  /// `attributes`
  AttributeList,
  /// `comment`
  Comment,
  /// `extra_ctx`
  ExtraCtx,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum GroupType {
  #[default]
  Option,
  Set,
  Vec,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum SimpleType {
  #[default]
  Default,
  Option,
}

#[derive(Debug, Clone, Copy, Default)]
pub(crate) enum ComplexType {
  #[default]
  Default,
  Option,
  Vec,
  Set,
}

#[derive(Debug, Clone)]
pub(crate) enum AttriType {
  /// `simple`
  Simple(SimpleType),
  /// `complex`
  Complex(ComplexType),
  /// `group`
  Group(GroupType),
  /// `supergroup`
  SuperGroup(Vec<(Ident, Type)>),
}

#[derive(Debug, Clone)]
pub(crate) enum FieldType {
  /// `Internal`
  Internal(InternalType),
  /// `Attri`
  Attri(AttriType),
}

#[allow(clippy::type_complexity)]
pub(crate) fn parse_fields_type(
  fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> syn::Result<(
  // attri map
  HashMap<&Ident, AttriType>,
  // default map
  HashMap<&Ident, Expr>,
  // before_build_map
  HashMap<&Ident, MyPath>,
  // after_build_map
  HashMap<&Ident, MyPath>,
  // Name (flatten, field)
  Vec<(bool, &syn::Field)>,
  // attributes name
  &Ident,
  // comment name
  &Ident,
  // extra_ctx name
  &Ident,
  // extra_ctx type
  &Type,
)> {
  let mut _name_vec = Vec::new();
  let mut _attributes_name = None;
  let mut _comments_name = None;
  let mut _extra_ctx = None;
  let mut attri_type_map = HashMap::new();
  let mut default_map = HashMap::new();
  let mut before_build_map = HashMap::new();
  let mut after_build_map = HashMap::new();
  for field in fields {
    if let (Some(field_name), field_attrs) = (&field.ident, &field.attrs) {
      let attr = parse_field_attrs(field_attrs)?;
      if let Some(default) = parse_field_default(field_attrs)? {
        _ = default_map.insert(field_name, default);
      }
      match parse_field_build(field_attrs)? {
        (None, None) => {}
        (None, Some(after)) => _ = after_build_map.insert(field_name, after),
        (Some(before), None) => _ = before_build_map.insert(field_name, before),
        (Some(before), Some(after)) => {
          _ = before_build_map.insert(field_name, before);
          _ = after_build_map.insert(field_name, after);
        }
      }
      let attr = match attr {
        Some(t) => t,
        None => {
          return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("{}: Can not find #[liberty ...].", field_name),
          ));
        }
      };
      match attr {
        FieldType::Internal(InternalType::Name { flatten }) => {
          _name_vec.push((flatten, field));
        }
        FieldType::Internal(InternalType::AttributeList) => {
          if let Some(name) = &_attributes_name {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              format!("duplicated attributes {}.", name),
            ));
          } else {
            _attributes_name = Some(field_name);
          }
        }
        FieldType::Internal(InternalType::Comment) => {
          if let Some(name) = &_comments_name {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              format!("duplicated comment {}.", name),
            ));
          } else {
            _comments_name = Some(field_name);
          }
        }
        FieldType::Internal(InternalType::ExtraCtx) => {
          if let Some((name, _)) = &_extra_ctx {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              format!("duplicated extra_ctx {}.", name),
            ));
          } else {
            _extra_ctx = Some((field_name, &field.ty));
          }
        }
        FieldType::Attri(attri_type) => {
          _ = attri_type_map.insert(field_name, attri_type);
        }
      }
    } else {
      return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "field error.".to_string(),
      ));
    }
  }

  match (_attributes_name, _comments_name, _extra_ctx) {
    (None, _, _) => Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "Can not find attributes".to_string(),
    )),
    (_, None, _) => Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "Can not find comment".to_string(),
    )),
    (_, _, None) => Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "Can not find extra_ctx".to_string(),
    )),
    (
      Some(attributes_name),
      Some(comments_name),
      Some((extra_ctx_name, extra_ctx_type)),
    ) => Ok((
      attri_type_map,
      default_map,
      before_build_map,
      after_build_map,
      _name_vec,
      attributes_name,
      comments_name,
      extra_ctx_name,
      extra_ctx_type,
    )),
  }
}

/// ```
/// // Attributes
/// #[liberty(attributes)]
/// // GroupComments
/// #[liberty(comments)]
/// // Auto vector Id: Vec<String>
/// #[liberty(id(title=0))]
/// // Auto Id: Option<String>
/// #[liberty(id(title=0.5))]
/// // Auto Id: String
/// #[liberty(id(title=1))]
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
pub(crate) fn parse_field_attrs(
  field_attrs: &[syn::Attribute],
) -> syn::Result<Option<FieldType>> {
  for attri in field_attrs.iter() {
    if let Some(seg_title) = attri.path().segments.first() {
      if "liberty" == &seg_title.ident.to_string() {
        if let syn::Meta::List(list) = &attri.meta {
          let mut tokens: proc_macro2::token_stream::IntoIter =
            list.tokens.clone().into_iter();
          if let Some(proc_macro2::TokenTree::Ident(token_id)) = tokens.next() {
            match token_id.to_string().as_str() {
              "name" => {
                return Ok(Some(FieldType::Internal(InternalType::Name {
                  flatten: parse_name_flatten(tokens)?,
                })));
              }
              "attributes" => {
                return Ok(Some(FieldType::Internal(InternalType::AttributeList)));
              }
              "comments" => return Ok(Some(FieldType::Internal(InternalType::Comment))),
              "extra_ctx" => {
                return Ok(Some(FieldType::Internal(InternalType::ExtraCtx)));
              }
              "simple" => {
                let simple_type = parse_simple_type(tokens)?;
                return Ok(Some(FieldType::Attri(AttriType::Simple(simple_type))));
              }
              "complex" => {
                let complex_type = parse_complex_type(tokens)?;
                return Ok(Some(FieldType::Attri(AttriType::Complex(complex_type))));
              }
              "group" => {
                let group_type = parse_group_type(tokens)?;
                return Ok(Some(FieldType::Attri(AttriType::Group(group_type))));
              }
              "supergroup" => {
                return Ok(Some(FieldType::Attri(AttriType::SuperGroup(
                  parse_supergroup_type(tokens)?,
                ))));
              }
              "default" => {
                continue;
              }
              _ => {
                return Err(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("Unsupported token {}.", token_id.to_string().as_str()),
                ));
              }
            }
          } else {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              "No token.".to_string(),
            ));
          }
        } else {
          return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "Incorrect format for using the `liberty` attribute.".to_string(),
          ));
        }
      }
    }
  }
  Ok(None)
}
fn parse_field_default(field_attrs: &[syn::Attribute]) -> syn::Result<Option<Expr>> {
  for attr in field_attrs {
    if attr.path().is_ident("liberty") {
      let res = attr.parse_args_with(|input: ParseStream| {
        let mut default_expr = None;
        if input.is_empty() {
          return Ok(default_expr);
        }
        let key: Ident = input.parse()?;

        if key == "default" {
          let _eq: Token![=] = input.parse()?;
          let expr: Expr = input.parse()?;

          default_expr = Some(expr);
        }
        while !input.is_empty() {
          _ = input.parse::<proc_macro2::TokenStream>()?;
        }
        Ok(default_expr)
      });
      if let Ok(Some(expr)) = res {
        return Ok(Some(expr));
      }
    }
  }
  Ok(None)
}
fn parse_field_build(
  field_attrs: &[syn::Attribute],
) -> syn::Result<(Option<MyPath>, Option<MyPath>)> {
  let mut before_build_expr = None;
  let mut after_build_expr = None;
  for attr in field_attrs {
    if attr.path().is_ident("liberty") {
      attr.parse_args_with(|input: ParseStream| {
        if input.is_empty() {
          return Ok(());
        }
        let key: Ident = input.parse()?;
        if key == "before_build" {
          let _eq: Token![=] = input.parse()?;
          let expr: MyPath = input.parse()?;
          before_build_expr = Some(expr);
        }
        if key == "after_build" {
          let _eq: Token![=] = input.parse()?;
          let expr: MyPath = input.parse()?;
          after_build_expr = Some(expr);
        }
        while !input.is_empty() {
          _ = input.parse::<proc_macro2::TokenStream>()?;
        }
        Ok(())
      })?;
    }
  }
  Ok((before_build_expr, after_build_expr))
}

fn parse_name_flatten(
  mut tokens: proc_macro2::token_stream::IntoIter,
) -> syn::Result<bool> {
  let mut flatten = false;
  if let Some(proc_macro2::TokenTree::Group(g)) = tokens.next() {
    let mut args = g.stream().into_iter();
    while let Some(proc_macro2::TokenTree::Ident(arg_id)) = args.next() {
      match arg_id.to_string().as_str() {
        "flatten" => flatten = true,
        _ => {
          return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("name not support {}.", arg_id.to_string().as_str()),
          ));
        }
      }
    }
  }
  Ok(flatten)
}

fn parse_simple_type(
  mut tokens: proc_macro2::token_stream::IntoIter,
) -> syn::Result<SimpleType> {
  let mut simple_type = SimpleType::default();
  if let Some(proc_macro2::TokenTree::Group(g)) = tokens.next() {
    let mut args = g.stream().into_iter();
    while let Some(proc_macro2::TokenTree::Ident(arg_id)) = args.next() {
      match arg_id.to_string().as_str() {
        "type" => {
          if let Some(proc_macro2::TokenTree::Punct(arg_punct)) = args.next() {
            if '=' != arg_punct.as_char() {
              return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "miss equal.".to_string(),
              ));
            }
          } else {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              "miss equal.".to_string(),
            ));
          }
          if let Some(proc_macro2::TokenTree::Ident(arg_value)) = args.next() {
            match arg_value.to_string().as_str() {
              "Option" => simple_type = SimpleType::Option,
              "Default" => simple_type = SimpleType::Default,
              _ => {
                return Err(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("simple_type not support {}.", arg_value.to_string().as_str()),
                ));
              }
            }
          } else {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              "miss simple_type.".to_string(),
            ));
          }
        }
        _ => {
          return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("simple_type not support {} group.", arg_id.to_string().as_str()),
          ));
        }
      }
    }
  }
  Ok(simple_type)
}

fn parse_complex_type(
  mut tokens: proc_macro2::token_stream::IntoIter,
) -> syn::Result<ComplexType> {
  let mut complex_type = ComplexType::default();
  if let Some(proc_macro2::TokenTree::Group(g)) = tokens.next() {
    let mut args = g.stream().into_iter();
    while let Some(proc_macro2::TokenTree::Ident(arg_id)) = args.next() {
      match arg_id.to_string().as_str() {
        "type" => {
          if let Some(proc_macro2::TokenTree::Punct(arg_punct)) = args.next() {
            if '=' != arg_punct.as_char() {
              return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "miss equal.".to_string(),
              ));
            }
          } else {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              "miss equal.".to_string(),
            ));
          }
          if let Some(proc_macro2::TokenTree::Ident(arg_value)) = args.next() {
            match arg_value.to_string().as_str() {
              "Default" => complex_type = ComplexType::Default,
              "Option" => complex_type = ComplexType::Option,
              "Vec" => complex_type = ComplexType::Vec,
              "Set" => complex_type = ComplexType::Set,
              _ => {
                return Err(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("complex_type not support {}.", arg_value.to_string().as_str()),
                ));
              }
            }
          } else {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              "miss simple_type.".to_string(),
            ));
          }
        }
        _ => {
          return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("simple_type not support {} group.", arg_id.to_string().as_str()),
          ));
        }
      }
    }
  }
  Ok(complex_type)
}

fn parse_supergroup_type(
  tokens: proc_macro2::token_stream::IntoIter,
) -> syn::Result<Vec<(Ident, Type)>> {
  struct KeyValue {
    key: Ident,
    value: Type,
  }
  impl syn::parse::Parse for KeyValue {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
      let key = input.parse()?;
      let _: syn::Token![:] = input.parse()?;
      let value = input.parse()?;
      Ok(KeyValue { key, value })
    }
  }
  struct FieldsParser {
    fields: syn::punctuated::Punctuated<KeyValue, syn::Token![,]>,
  }
  impl syn::parse::Parse for FieldsParser {
    fn parse(input: syn::parse::ParseStream) -> syn::parse::Result<Self> {
      let content;
      syn::parenthesized!(content in input);
      let fields = syn::punctuated::Punctuated::parse_terminated(&content)?;
      Ok(FieldsParser { fields })
    }
  }
  let token = tokens.collect();
  let parser = syn::parse2::<FieldsParser>(token)?;
  Ok(parser.fields.into_iter().map(|kv| (kv.key, kv.value)).collect())
}
fn parse_group_type(
  mut tokens: proc_macro2::token_stream::IntoIter,
) -> syn::Result<GroupType> {
  let mut group_type = GroupType::default();
  if let Some(proc_macro2::TokenTree::Group(g)) = tokens.next() {
    let mut args = g.stream().into_iter();
    while let Some(proc_macro2::TokenTree::Ident(arg_id)) = args.next() {
      match arg_id.to_string().as_str() {
        "type" => {
          if let Some(proc_macro2::TokenTree::Punct(arg_punct)) = args.next() {
            if '=' != arg_punct.as_char() {
              return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "miss equal.".to_string(),
              ));
            }
          } else {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              "miss equal.".to_string(),
            ));
          }
          if let Some(proc_macro2::TokenTree::Ident(arg_value)) = args.next() {
            match arg_value.to_string().as_str() {
              "Option" => group_type = GroupType::Option,
              "Vec" => group_type = GroupType::Vec,
              "Set" => group_type = GroupType::Set,
              _ => {
                return Err(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("group_type not support {}.", arg_value.to_string().as_str()),
                ));
              }
            }
          } else {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              "miss group_type.".to_string(),
            ));
          }
        }
        _ => {
          return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("group_type not support {} group.", arg_id.to_string().as_str()),
          ));
        }
      }
    }
  }
  Ok(group_type)
}

#[test]
fn size_type_test() {
  use quote::ToTokens as _;
  // let attr: Attribute = parse_quote!(#[id]);
  // let attr: Attribute = parse_quote!(#[id(borrow="&[String]")] );
  let attr: Vec<syn::Attribute> = syn::parse_quote!(
    #[liberty(group(type = Set))]
    #[liberty(default = vec![0.0])]);
  let s = parse_field_default(&attr).unwrap().unwrap();
  println!("{}", s.to_token_stream());
  // let t: proc_macro2::TokenStream = syn::parse_str(&s).unwrap();
  // println!("{t:?}");
}
#[test]
fn build_test() {
  // use quote::ToTokens as _;
  // let attr: Attribute = parse_quote!(#[id]);
  // let attr: Attribute = parse_quote!(#[id(borrow="&[String]")] );
  let aaa: MyPath = syn::parse_quote!(vec!);
  println!("{:?}", aaa);
  let aaa: MyPath = syn::parse_quote!(vec);
  println!("{:?}", aaa);
  // let attr: Vec<syn::Attribute> = syn::parse_quote!(
  //   #[liberty(group(type = Set))]
  //   #[liberty(before_build = vec!)]
  //   #[liberty(after_build = vec![1.0])]
  // );
  // let (before, after) = parse_field_build(&attr).unwrap();
  // println!("{}", before.unwrap().0.to_token_stream());
  // println!("{}", after.unwrap().0.to_token_stream());
  // let t: proc_macro2::TokenStream = syn::parse_str(&s).unwrap();
  // println!("{t:?}");
}

#[derive(Debug)]
pub(crate) struct MyPath {
  path: Path,
  pub is_macro: bool,
}
impl ToTokens for MyPath {
  fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
    tokens.extend(self.path.to_token_stream());
    if self.is_macro {
      tokens.extend(quote! {!});
    }
  }
}

impl Parse for MyPath {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let path = input.parse::<Path>()?;
    let is_macro = input.peek(Token![!]);
    if is_macro {
      // 消费掉 "!" 令牌
      input.parse::<Token![!]>()?;
    }
    Ok(MyPath { path, is_macro })
  }
}

#[test]
fn main() {
  use syn::{Data, parse_str};
  let input = r#"
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
#[serde(bound = "C::Other: serde::Serialize + serde::de::DeserializeOwned")]
pub struct Foo {
  #[liberty(attributes)]
  attributes: bool,
    #[liberty(id(title=2))]
    id_2: bool,
    #[liberty(id(title=0))]
    id_vec: bool,
    #[liberty(id)]
    id_no_impl: bool,
    #[liberty(simple)]
    simple: i64,
    #[liberty(complex)]
    complex: i64,
    #[liberty(other)]
    other: i64,
    #[liberty(group(type=Map))]
    group_map: i64,
    #[liberty(group(type = Option))]
    group_option: i64,
    #[liberty(group(type=Vec))]
    group_vec: i64,
    #[liberty(supergroup(
      cell_fall: Option<TableLookUp<C>>,
      ocv_mean_shift_cell_fall: Option<TableLookUp<C>>,
      ocv_std_dev_cell_fall: Option<TableLookUp<C>>,
      ocv_skewness_cell_fall: Option<TableLookUp<C>>,
    ))]
    pub cell_fall: Option<TimingTableLookUp>,
}"#;
  let ast: &syn::DeriveInput = &parse_str(input).unwrap();
  if let Data::Struct(st) = &ast.data {
    if let syn::Fields::Named(named) = &st.fields {
      let fields = &named.named;
      let got: Vec<(String, syn::Result<Option<FieldType>>)> = fields
        .into_iter()
        .map(|field| {
          if let (Some(field_name), field_attrs) = (&field.ident, &field.attrs) {
            (field_name.to_string(), parse_field_attrs(field_attrs))
          } else {
            panic!("");
          }
        })
        .collect();
      // println!("{:?}", want);
      println!("{:?}", got);
      // assert_eq!(format!("{:?}", got), format!("{:?}", want));
    };
  };
}
