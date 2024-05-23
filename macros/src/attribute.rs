use std::{collections::HashMap, fmt::Debug};

use proc_macro2::Ident;

#[derive(Debug, Clone, Copy)]
enum InternalType {
  /// `name`
  Name,
  /// `undefined`
  UndefinedAttributeList,
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

pub(crate) fn parse_fields_type(
  fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> syn::Result<(
  HashMap<&Ident, AttriType>,
  // Name
  Vec<&syn::Field>,
  // undefined name
  &Ident,
  // comment name
  &Ident,
)> {
  let mut _name_vec = Vec::new();
  let mut _undefined_name = None;
  let mut _comments_name = None;
  let mut err_buf = None;
  let attri_type_map: HashMap<&Ident, AttriType> = fields
    .iter()
    .filter_map(|field| {
      if let (Some(field_name), field_attrs) = (&field.ident, &field.attrs) {
        match parse_field_attrs(field_attrs) {
          Ok(Some(t)) => match t {
            FieldType::Internal(InternalType::Name) => {
              _name_vec.push(field);
              // if let Some(name) = &_name_name {
              //   err_buf = Some(syn::Error::new(
              //     proc_macro2::Span::call_site(),
              //     format!("duplicated name {}.", name),
              //   ));
              // } else {
              //   _name_name = Some(field_name);
              // }
              None
            }
            FieldType::Internal(InternalType::UndefinedAttributeList) => {
              if let Some(name) = &_undefined_name {
                err_buf = Some(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("duplicated undefined {}.", name),
                ));
              } else {
                _undefined_name = Some(field_name);
              }
              None
            }
            FieldType::Internal(InternalType::Comment) => {
              if let Some(name) = &_comments_name {
                err_buf = Some(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("duplicated comment {}.", name),
                ));
              } else {
                _comments_name = Some(field_name);
              }
              None
            }
            FieldType::Attri(attri_type) => Some((field_name, attri_type)),
          },
          Ok(None) => None,
          Err(e) => {
            err_buf = Some(e);
            None
          }
        }
      } else {
        err_buf =
          Some(syn::Error::new(proc_macro2::Span::call_site(), format!("field error.")));
        None
      }
    })
    .collect();
  if let Some(e) = err_buf {
    return Err(e);
  } else {
    match (_undefined_name, _comments_name) {
      (None, None) => {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          format!("Can not find undefined & comment"),
        ))
      }
      (None, Some(_)) => {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          format!("Can not find undefined"),
        ))
      }
      (Some(_), None) => {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          format!("Can not find comment"),
        ))
      }
      (Some(undefined_name), Some(comments_name)) => {
        return Ok((attri_type_map, _name_vec, undefined_name, comments_name))
      }
    }
  }
}

/// ```
/// // UndefinedAttribute
/// #[liberty(undefined)]
/// // GroupComments
/// #[liberty(comments)]
/// // Auto vector Id: Vec<FastStr>
/// #[liberty(id(title=0))]
/// // Auto Id: Option<FastStr>
/// #[liberty(id(title=0.5))]
/// // Auto Id: FastStr
/// #[liberty(id(title=1))]
/// // Auto slice Id: [FastStr:2]
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
fn parse_field_attrs(
  field_attrs: &Vec<syn::Attribute>,
) -> syn::Result<Option<FieldType>> {
  for attri in field_attrs.into_iter() {
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
              "undefined" => {
                return Ok(Some(FieldType::Internal(
                  InternalType::UndefinedAttributeList,
                )))
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
              format!("No token."),
            ));
          }
        } else {
          return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            format!("Incorrect format for using the `liberty` attribute."),
          ));
        }
      }
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
    loop {
      if let Some(proc_macro2::TokenTree::Ident(arg_id)) = args.next() {
        match arg_id.to_string().as_str() {
          "type" => {
            if let Some(proc_macro2::TokenTree::Punct(arg_punct)) = args.next() {
              if '=' != arg_punct.as_char() {
                return Err(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("miss equal."),
                ));
              }
            } else {
              return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("miss equal."),
              ));
            }
            if let Some(proc_macro2::TokenTree::Ident(arg_value)) = args.next() {
              match arg_value.to_string().as_str() {
                "Option" => simple_type = SimpleType::Option,
                "Default" => simple_type = SimpleType::Default,
                _ => {
                  return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    format!(
                      "simple_type not support {}.",
                      arg_value.to_string().as_str()
                    ),
                  ))
                }
              }
            } else {
              return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("miss simple_type."),
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
      } else {
        break;
      }
    }
  }
  return Ok(simple_type);
}

fn parse_complex_type(
  mut tokens: proc_macro2::token_stream::IntoIter,
) -> syn::Result<ComplexType> {
  let mut complex_type = ComplexType::default();
  if let Some(proc_macro2::TokenTree::Group(g)) = tokens.next() {
    let mut args = g.stream().into_iter();
    loop {
      if let Some(proc_macro2::TokenTree::Ident(arg_id)) = args.next() {
        match arg_id.to_string().as_str() {
          "type" => {
            if let Some(proc_macro2::TokenTree::Punct(arg_punct)) = args.next() {
              if '=' != arg_punct.as_char() {
                return Err(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("miss equal."),
                ));
              }
            } else {
              return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("miss equal."),
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
                    format!(
                      "complex_type not support {}.",
                      arg_value.to_string().as_str()
                    ),
                  ))
                }
              }
            } else {
              return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("miss simple_type."),
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
      } else {
        break;
      }
    }
  }
  return Ok(complex_type);
}

fn parse_group_type(
  mut tokens: proc_macro2::token_stream::IntoIter,
) -> syn::Result<GroupType> {
  let mut group_type = GroupType::default();
  if let Some(proc_macro2::TokenTree::Group(g)) = tokens.next() {
    let mut args = g.stream().into_iter();
    loop {
      if let Some(proc_macro2::TokenTree::Ident(arg_id)) = args.next() {
        match arg_id.to_string().as_str() {
          "type" => {
            if let Some(proc_macro2::TokenTree::Punct(arg_punct)) = args.next() {
              if '=' != arg_punct.as_char() {
                return Err(syn::Error::new(
                  proc_macro2::Span::call_site(),
                  format!("miss equal."),
                ));
              }
            } else {
              return Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                format!("miss equal."),
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
                format!("miss group_type."),
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
      } else {
        break;
      }
    }
  }
  return Ok(group_type);
}

#[test]
fn main() {
  use syn::{parse_str, Data};
  let input = r#"
#[derive(liberty_macros::Group)]
#[derive(serde::Serialize, serde::Deserialize)]
pub struct Foo {
  #[liberty(undefined)]
    undefined: bool,
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
