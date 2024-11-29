extern crate proc_macro;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use syn::{Data, DeriveInput, Meta};

pub(crate) fn inner(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let enum_name = &input.ident;
  let iter_name = syn::Ident::new(&format!("Iter{enum_name}"), Span::call_site());
  let test_mod_name = syn::Ident::new(
    &format!("_enum_test_{}", to_snake_case(&enum_name.to_string())),
    Span::call_site(),
  );
  let data_enum = match &input.data {
    Data::Enum(data_enum) => data_enum,
    _ => {
      return Err(syn::Error::new(
        Span::call_site(),
        "EnumParseDisplay can only be used with enums".to_string(),
      ))
    }
  };

  let mut parse_match_quotes = Vec::new();
  let mut fmt_match_quotes = Vec::new();
  let mut iter_match_quotes = Vec::new();
  let mut iter_first_quote = quote! {};
  let mut all_tokens = Vec::new();
  let mut last_variant = None;
  for variant in &data_enum.variants {
    let variant_name = &variant.ident;
    let mut tokens = Vec::new();

    for attr in &variant.attrs {
      if attr.path().is_ident("token") {
        match &attr.meta {
          Meta::List(meta_list) => {
            for token in meta_list.tokens.clone() {
              if let TokenTree::Literal(ref lit) = token {
                let s = lit.to_string();
                let unquote = &s[1..s.len() - 1];
                tokens.push(unquote.to_string());
              }
            }
          }
          Meta::Path(_) | Meta::NameValue(_) => {
            return Err(syn::Error::new(
              Span::call_site(),
              format!("{variant_name}: Expected string literal"),
            ))
          }
        }
      }
    }
    if tokens.is_empty() {
      return Err(syn::Error::new(
        Span::call_site(),
        format!("{variant_name}: No token"),
      ));
    }
    if let Some((last_enum_name, last_variant_name)) = last_variant {
      iter_match_quotes.push(quote! {
        Some(#last_enum_name::#last_variant_name) => self.0 = Some(#enum_name::#variant_name),
      });
    } else {
      iter_first_quote = quote! {Some(Self::#variant_name)}
    }
    last_variant = Some((enum_name, variant_name));
    let last_token = tokens.last().unwrap();
    if tokens.len() > 1 {
      parse_match_quotes.push(quote! {
        map(alt((#( tag(#tokens) ),*)), |_| #enum_name::#variant_name),
      });
    } else {
      parse_match_quotes.push(quote! {
        map(#( tag(#tokens) ),*, |_| #enum_name::#variant_name),
      });
    }
    fmt_match_quotes.push(quote! {
      #enum_name::#variant_name => write!(f, #last_token),
    });
    all_tokens.extend(tokens);
  }
  if let Some((last_enum_name, last_variant_name)) = last_variant {
    iter_match_quotes.push(quote! {
      Some(#last_enum_name::#last_variant_name) => self.0 = None,
    });
  }

  Ok(quote! {
    pub struct #iter_name(Option<#enum_name>);
    impl #enum_name {
      pub fn iter() -> #iter_name {
        #iter_name(#iter_first_quote)
      }
    }
    impl Iterator for #iter_name {
      type Item = #enum_name;
      #[inline]
      fn next(&mut self) -> Option<Self::Item> {
        let res = self.0;
        match self.0 {
          #(#iter_match_quotes)*
          None => (),
        }
        res
      }
    }
    const _:() = {
      use nom::{
        error::Error, IResult,
        branch::alt, bytes::complete::tag, character::complete::char, combinator::map,
        sequence::delimited,
      };
      impl crate::ast::NomParseTerm for #enum_name {
        fn nom_parse<'a>(i: &'a str) -> IResult<&'a str, Self, Error<&'a str>> {
          alt((
            delimited(
              char('"'),
              alt((
                #(#parse_match_quotes)*
              )),
              char('"'),
            ),
            #(#parse_match_quotes)*
          ))(i)
        }
      }
      impl core::fmt::Display for #enum_name {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
          match self {
            #(#fmt_match_quotes)*
          }
        }
      }
    };
    #[cfg(test)]
    mod #test_mod_name {
      #[test]
      fn parse() {
        for s in [
          #(#all_tokens),*
        ] {
          let (_, v) = <super::#enum_name as crate::ast::NomParseTerm>::nom_parse(s).unwrap();
          assert_eq!(s, &v.to_string());
        }
      }
    }
  })
}
fn to_snake_case(s: &str) -> String {
  let mut snake_case = String::new();
  let chars: Vec<char> = s.chars().collect();
  for (i, &c) in chars.iter().enumerate() {
    if c.is_uppercase() && i != 0 {
      snake_case.push('_');
    }
    snake_case.push(c.to_lowercase().next().unwrap());
  }
  snake_case
}
#[test]
fn test() {
  use syn::parse_str;
  let input = r#"
  #[derive(liberty_macros::EnumToken)]
enum OneValue {
  #[token("1")]
  One,
  #[token("0")]
  Zero,
  #[token("x")]
  Unkown,
}"#;
  let ast: &syn::DeriveInput = &parse_str(input).unwrap();
  let out = inner(ast).unwrap_or_else(|err| err.to_compile_error());
  println!("{}", out.to_string())
}
