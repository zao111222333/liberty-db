use core::fmt::Debug;
use proc_macro2::Ident;
use std::collections::HashMap;
use syn::{Expr, Path, Token, Type, parse::ParseStream};

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

#[derive(Debug, Clone)]
pub(crate) enum AttriType {
  /// `simple`
  Simple,
  /// `complex`
  Complex,
  /// `group`
  Group,
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

pub(crate) struct FieldsType<'a> {
  // attri map
  pub(crate) attri_type_map: HashMap<&'a Ident, AttriType>,
  /// default map
  pub(crate) default_map: HashMap<&'a Ident, Expr>,
  /// dynamic name map
  pub(crate) dynamic_name_map: HashMap<&'a Ident, Expr>,
  /// before_build_map
  pub(crate) before_build_map: HashMap<&'a Ident, Path>,
  /// after_build_map
  pub(crate) after_build_map: HashMap<&'a Ident, Path>,
  /// Name (flatten, field)
  pub(crate) name_vec: Vec<(bool, &'a syn::Field)>,
  /// attributes name
  pub(crate) attributes_name: &'a Ident,
  /// comment name
  pub(crate) comments_name: &'a Ident,
  /// extra_ctx name
  pub(crate) extra_ctx_name: &'a Ident,
}

#[allow(clippy::type_complexity)]
pub(crate) fn parse_fields_type(
  fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> syn::Result<FieldsType<'_>> {
  let mut name_vec = Vec::new();
  let mut _attributes_name = None;
  let mut _comments_name = None;
  let mut _extra_ctx_name = None;
  let mut attri_type_map = HashMap::new();
  let mut default_map = HashMap::new();
  let mut dynamic_name_map = HashMap::new();
  let mut before_build_map = HashMap::new();
  let mut after_build_map = HashMap::new();
  for field in fields {
    if let (Some(field_name), field_attrs) = (&field.ident, &field.attrs) {
      let attr = parse_field_attrs(field_attrs)?;
      if let Some(default) = parse_field_default(field_attrs)? {
        _ = default_map.insert(field_name, default);
      }
      if let Some(default) = parse_field_dynamic_name(field_attrs)? {
        _ = dynamic_name_map.insert(field_name, default);
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
            format!("{field_name}: Can not find #[liberty ...]."),
          ));
        }
      };
      match attr {
        FieldType::Internal(InternalType::Name { flatten }) => {
          name_vec.push((flatten, field));
        }
        FieldType::Internal(InternalType::AttributeList) => {
          if let Some(name) = &_attributes_name {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              format!("duplicated attributes {name}."),
            ));
          } else {
            _attributes_name = Some(field_name);
          }
        }
        FieldType::Internal(InternalType::Comment) => {
          if let Some(name) = &_comments_name {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              format!("duplicated comment {name}."),
            ));
          } else {
            _comments_name = Some(field_name);
          }
        }
        FieldType::Internal(InternalType::ExtraCtx) => {
          if let Some(name) = &_extra_ctx_name {
            return Err(syn::Error::new(
              proc_macro2::Span::call_site(),
              format!("duplicated extra_ctx {name}."),
            ));
          } else {
            _extra_ctx_name = Some(field_name);
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

  match (_attributes_name, _comments_name, _extra_ctx_name) {
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
    (Some(attributes_name), Some(comments_name), Some(extra_ctx_name)) => {
      Ok(FieldsType {
        attri_type_map,
        default_map,
        dynamic_name_map,
        before_build_map,
        after_build_map,
        name_vec,
        attributes_name,
        comments_name,
        extra_ctx_name,
      })
    }
  }
}

pub(crate) fn parse_field_attrs(
  field_attrs: &[syn::Attribute],
) -> syn::Result<Option<FieldType>> {
  for attri in field_attrs.iter() {
    if let Some(seg_title) = attri.path().segments.first()
      && "liberty" == &seg_title.ident.to_string()
    {
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
              return Ok(Some(FieldType::Attri(AttriType::Simple)));
            }
            "complex" => {
              return Ok(Some(FieldType::Attri(AttriType::Complex)));
            }
            "group" => {
              return Ok(Some(FieldType::Attri(AttriType::Group)));
            }
            "supergroup" => {
              return Ok(Some(FieldType::Attri(AttriType::SuperGroup(
                parse_supergroup_type(tokens)?,
              ))));
            }
            "default" | "dynamic_name" => {
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
fn parse_field_dynamic_name(field_attrs: &[syn::Attribute]) -> syn::Result<Option<Expr>> {
  for attr in field_attrs {
    if attr.path().is_ident("liberty") {
      let res = attr.parse_args_with(|input: ParseStream| {
        let mut default_expr = None;
        if input.is_empty() {
          return Ok(default_expr);
        }
        let key: Ident = input.parse()?;

        if key == "dynamic_name" {
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
) -> syn::Result<(Option<Path>, Option<Path>)> {
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
          let expr: Path = input.parse()?;
          before_build_expr = Some(expr);
        }
        if key == "after_build" {
          let _eq: Token![=] = input.parse()?;
          let expr: Path = input.parse()?;
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
