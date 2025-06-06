use std::mem;

use quote::quote;
// use proc_macro::Ident;
use syn::{
  Attribute, Data, DeriveInput, Field, Fields, Ident, Token,
  parse::{Parse, ParseStream},
  punctuated::Punctuated,
  spanned::Spanned,
};

use crate::attribute::{AttriType, FieldType, parse_field_attrs};
#[derive(Debug)]
struct Config {
  name: Ident,
  additional_attrs: Punctuated<Field, Token![,]>,
  docs: Vec<Attribute>,
  exclude_simple: bool,
  exclude_complex: bool,
  exclude_group: bool,
}

#[derive(Debug)]
enum DuplicatedArg {
  Name(Ident),
  Docs(Vec<Attribute>),
  Exclude(Punctuated<Ident, Token![,]>),
  NewAttrs(Punctuated<Field, Token![,]>),
}

impl Parse for DuplicatedArg {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    let lookahead = input.lookahead1();
    if lookahead.peek(Ident) {
      let key: Ident = input.parse()?;
      let key_str = key.to_string();
      match key_str.as_str() {
        "name" => {
          input.parse::<Token![=]>()?;
          let value: Ident = input.parse()?;
          Ok(DuplicatedArg::Name(value))
        }
        "additional_attrs" => {
          let content;
          syn::parenthesized!(content in input);
          let fields = Punctuated::parse_terminated_with(&content, Field::parse_named)?;
          Ok(DuplicatedArg::NewAttrs(fields))
        }
        "exclude" => {
          let content;
          syn::parenthesized!(content in input);
          let excludes = Punctuated::parse_terminated_with(&content, Ident::parse)?;
          Ok(DuplicatedArg::Exclude(excludes))
        }
        "docs" => {
          let content;
          syn::parenthesized!(content in input);
          let docs = Attribute::parse_outer(&content)?;
          // Punctuated::parse_terminated_with(&content, Attribute::parse_outer)?;
          Ok(DuplicatedArg::Docs(docs))
        }
        _ => Err(syn::Error::new(
          key.span(),
          "Only support `name = ...` or `additional_attrs(...)`",
        )),
      }
    } else {
      Err(lookahead.error())
    }
  }
}

#[derive(Debug)]
struct Args {
  args: Punctuated<DuplicatedArg, Token![,]>,
}

impl Parse for Args {
  fn parse(input: ParseStream) -> syn::Result<Self> {
    Ok(Self {
      args: Punctuated::parse_terminated_with(&input, DuplicatedArg::parse)?,
    })
  }
}

fn attrs_config(attrs: &[Attribute]) -> syn::Result<Vec<Config>> {
  let mut configs = Vec::new();
  for attr in attrs {
    if attr.path().is_ident("duplicated") {
      if let syn::Meta::List(list) = &attr.meta {
        let tokens: proc_macro2::token_stream::IntoIter = list.tokens.clone().into_iter();
        let args: Args = syn::parse2(tokens.clone().collect())?;

        let mut maybe_name: Option<Ident> = None;
        let mut maybe_new_attrs: Option<Punctuated<Field, Token![,]>> = None;
        let mut maybe_docs: Option<Vec<Attribute>> = None;
        let mut exclude_simple = false;
        let mut exclude_complex = false;
        let mut exclude_group = false;
        for arg in args.args {
          match arg {
            DuplicatedArg::Name(ident) => {
              if maybe_name.is_some() {
                return Err(syn::Error::new(ident.span(), "duplicated `name` attribute"));
              }
              maybe_name = Some(ident);
            }
            DuplicatedArg::NewAttrs(fields) => {
              if maybe_new_attrs.is_some() {
                return Err(syn::Error::new(
                  fields.span(),
                  "duplicated `additional_attrs` attribute",
                ));
              }
              maybe_new_attrs = Some(fields);
            }
            DuplicatedArg::Docs(docs) => {
              if maybe_new_attrs.is_some() {
                return Err(syn::Error::new(
                  docs[0].span(),
                  "duplicated `docs` attribute",
                ));
              }
              maybe_docs = Some(docs);
            }
            DuplicatedArg::Exclude(excludes) => {
              for exclude in excludes {
                match exclude.to_string().as_str() {
                  "simple" => exclude_simple = true,
                  "complex" => exclude_complex = true,
                  "group" => exclude_group = true,
                  _ => {
                    return Err(syn::Error::new(
                      exclude.span(),
                      format!("unsupport exclude {exclude}"),
                    ));
                  }
                }
              }
            }
          }
        }
        let name = maybe_name.ok_or_else(|| {
          syn::Error::new(proc_macro2::Span::call_site(), "miss `name = ...`")
        })?;
        let additional_attrs = maybe_new_attrs.ok_or_else(|| {
          syn::Error::new(proc_macro2::Span::call_site(), "miss `additional_attrs(...)`")
        })?;
        let docs = maybe_docs.ok_or_else(|| {
          syn::Error::new(proc_macro2::Span::call_site(), "miss `docs(...)`")
        })?;
        configs.push(Config {
          name,
          additional_attrs,
          docs,
          exclude_simple,
          exclude_complex,
          exclude_group,
        });
      }
    }
  }

  Ok(configs)
}

pub(crate) fn inner(ast: DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let configs = attrs_config(&ast.attrs)?;
  let mut asts = Vec::new();
  for mut config in configs {
    let mut ast = ast.clone();
    ast
      .attrs
      .retain(|attr| !attr.path().is_ident("duplicated") && !attr.path().is_ident("doc"));
    ast.ident = config.name;
    ast.attrs.extend(config.docs);
    let st = match &mut ast.data {
      Data::Struct(s) => s,
      _ => {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          "This macro only supports struct.",
        ));
      }
    };
    if let Fields::Named(named) = &mut st.fields {
      for f in mem::take(&mut named.named) {
        match parse_field_attrs(&f.attrs)? {
          Some(FieldType::Attri(AttriType::Simple(_))) => {
            if config.exclude_simple {
              continue;
            }
          }
          Some(FieldType::Attri(AttriType::Complex(_))) => {
            if config.exclude_complex {
              continue;
            }
          }
          Some(FieldType::Attri(AttriType::Group(_))) => {
            if config.exclude_group {
              continue;
            }
          }
          _ => {}
        }
        config.additional_attrs.push(f);
      }
      named.named = config.additional_attrs;
      asts.push(ast);
    } else {
      return Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "Can not find NamedField".to_string(),
      ));
    }
  }
  Ok(quote! {#(#asts)*})
}

#[test]
fn test_attrs_config() {
  let input = r#"
  #[duplicated(
    name = A1,
    docs(
      /// comment1
      /// comment2
    ),
    exclude(complex, group),
    additional_attrs(
      /// comment1
      #[liberty(simple)]
      pub foo1: T1,
      /// comment2
      pub foo2: T2,
    ),
  )]
  pub(crate) struct Timing<C: Ctx> {}"#;
  let ast: syn::DeriveInput = syn::parse_str(input).unwrap();
  let config = attrs_config(&ast.attrs).unwrap();
  dbg!(config);
}
