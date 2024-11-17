use core::fmt::Debug;
use std::collections::HashMap;

use proc_macro2::Ident;
use syn::spanned::Spanned;

#[derive(Debug, Clone, Copy)]
enum InternalType {
  /// `name`
  Name,
  /// `attributes`
  AttributeList,
  /// `comment`
  Comment,
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

#[derive(Debug, Clone, Copy)]
pub(crate) enum AttriType {
  /// `simple`
  Simple(SimpleType),
  /// `complex`
  Complex(ComplexType),
  /// `group`
  Group(GroupType),
}

#[derive(Debug, Clone, Copy)]
enum FieldType {
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
  HashMap<&Ident, (AttriType, Option<usize>)>,
  // default map
  HashMap<&Ident, proc_macro2::TokenStream>,
  // Name
  Vec<&syn::Field>,
  // attributes name
  &Ident,
  // comment name
  &Ident,
)> {
  let mut _name_vec = Vec::new();
  let mut _attributes_name = None;
  let mut _comments_name = None;
  let mut attri_type_map = HashMap::new();
  let mut default_map = HashMap::new();
  for field in fields {
    if let (Some(field_name), field_attrs) = (&field.ident, &field.attrs) {
      let pos = parse_field_pos(field_attrs)?;
      let attr = parse_field_attrs(field_attrs)?;
      if let Some(default) = parse_field_default(field_attrs)? {
        _ = default_map.insert(field_name, default);
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
        FieldType::Internal(InternalType::Name) => {
          _name_vec.push(field);
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
        FieldType::Attri(attri_type) => {
          _ = attri_type_map.insert(field_name, (attri_type, pos));
        }
      }
    } else {
      return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "field error.".to_string(),
      ));
    }
  }

  match (_attributes_name, _comments_name) {
    (None, None) => Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "Can not find attributes & comment".to_string(),
    )),
    (None, Some(_)) => Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "Can not find attributes".to_string(),
    )),
    (Some(_), None) => Err(syn::Error::new(
      proc_macro2::Span::call_site(),
      "Can not find comment".to_string(),
    )),
    (Some(attributes_name), Some(comments_name)) => {
      Ok((attri_type_map, default_map, _name_vec, attributes_name, comments_name))
    }
  }
}

/// ```
/// // Attributes
/// #[liberty(attributes)]
/// // GroupComments
/// #[liberty(comments)]
/// // Auto vector Id: Vec<ArcStr>
/// #[liberty(id(title=0))]
/// // Auto Id: Option<ArcStr>
/// #[liberty(id(title=0.5))]
/// // Auto Id: ArcStr
/// #[liberty(id(title=1))]
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
fn parse_field_attrs(field_attrs: &[syn::Attribute]) -> syn::Result<Option<FieldType>> {
  for attri in field_attrs.iter() {
    if let Some(seg_title) = attri.path().segments.first() {
      if "liberty" == &seg_title.ident.to_string() {
        if let syn::Meta::List(list) = &attri.meta {
          let mut tokens: proc_macro2::token_stream::IntoIter =
            list.tokens.clone().into_iter();
          if let Some(proc_macro2::TokenTree::Ident(token_id)) = tokens.next() {
            match token_id.to_string().as_str() {
              "name" => {
                return Ok(Some(FieldType::Internal(InternalType::Name)));
              }
              "attributes" => {
                return Ok(Some(FieldType::Internal(InternalType::AttributeList)))
              }
              "comments" => return Ok(Some(FieldType::Internal(InternalType::Comment))),
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
              _ => {
                return Err(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("Unsupported token {}.", token_id.to_string().as_str()),
                ))
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
fn parse_field_pos(field_attrs: &[syn::Attribute]) -> syn::Result<Option<usize>> {
  for attr in field_attrs {
    if attr.path().is_ident("old_pos") {
      match &attr.meta {
        syn::Meta::List(_) | syn::Meta::Path(_) => {
          return Err(syn::Error::new(attr.meta.span(), "expected #[old_pos = 123 ]"))
        }
        syn::Meta::NameValue(s) => {
          if let syn::Expr::Lit(expr_lit) = &s.value {
            if let syn::Lit::Int(lit_int) = &expr_lit.lit {
              return Ok(Some(lit_int.base10_parse::<usize>()?));
            }
          }
          return Err(syn::Error::new(attr.meta.span(), "Expected integer literal"));
        }
      };
    }
  }
  Ok(None)
}
fn parse_field_default(
  field_attrs: &[syn::Attribute],
) -> syn::Result<Option<proc_macro2::TokenStream>> {
  for attr in field_attrs {
    if attr.path().is_ident("default") {
      match &attr.meta {
        syn::Meta::List(_) | syn::Meta::Path(_) => {
          return Err(syn::Error::new(attr.meta.span(), "expected #[default = \"123\" ]"))
        }
        syn::Meta::NameValue(s) => {
          if let syn::Expr::Lit(expr_lit) = &s.value {
            if let syn::Lit::Str(lit_str) = &expr_lit.lit {
              return Ok(Some(syn::parse_str(&lit_str.value())?));
            }
          }
          return Err(syn::Error::new(
            attr.meta.span(),
            "Expected String literal, #[default = \"123\" ]",
          ));
        }
      };
    }
  }
  Ok(None)
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
                ))
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
          ))
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
                ))
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
          ))
        }
      }
    }
  }
  Ok(complex_type)
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
                ))
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
          ))
        }
      }
    }
  }
  Ok(group_type)
}

#[test]
fn size_type_test() {
  // let attr: Attribute = parse_quote!(#[id]);
  // let attr: Attribute = parse_quote!(#[id(borrow="&[ArcStr]")] );
  let attr: syn::Attribute =
    syn::parse_quote!(#[default = "arcstr::literal!(\"undefined\")"]);
  let s = dbg!(parse_field_default(&[attr])).unwrap().unwrap();
  println!("{s}");
  // let t: proc_macro2::TokenStream = syn::parse_str(&s).unwrap();
  // println!("{t:?}");
}

#[test]
fn main() {
  use syn::{parse_str, Data};
  let input = r#"
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
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
