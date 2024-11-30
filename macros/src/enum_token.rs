extern crate proc_macro;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use syn::{Data, DeriveInput, Meta};

pub(crate) fn inner(input: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let enum_name = &input.ident;
  let data_enum = match &input.data {
    Data::Enum(data_enum) => data_enum,
    _ => {
      return Err(syn::Error::new(
        Span::call_site(),
        "EnumParseDisplay can only be used with enums".to_string(),
      ))
    }
  };
  let mut name_tokens_list = Vec::new();
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
    name_tokens_list.push((quote! (#enum_name::#variant_name), tokens));
  }
  Ok(enum_quote(enum_name, name_tokens_list))
}

pub(crate) fn enum_quote(
  enum_name: &syn::Ident,
  name_tokens_list: Vec<(proc_macro2::TokenStream, Vec<String>)>,
) -> proc_macro2::TokenStream {
  let mut trie_nodes = Vec::new();
  let mut fmt_match_quotes = Vec::new();
  let mut iter_match_quotes = Vec::new();
  let mut iter_first_quote = quote! {};
  let mut all_tokens = Vec::new();
  let mut last_name = None;
  for (name, tokens) in name_tokens_list {
    if let Some(last_name) = last_name {
      iter_match_quotes.push(quote! {
        Some(#last_name) => self.0 = Some(#name),
      });
    } else {
      iter_first_quote = quote! {Some(#name)}
    }
    let last_token = tokens.last().unwrap();
    fmt_match_quotes.push(quote! {
      #name => write!(f, #last_token),
    });
    trie_nodes.extend(tokens.clone().into_iter().map(|s| (name.clone(), s)));
    all_tokens.push(tokens.last().unwrap().clone());
    last_name = Some(name);
  }
  if let Some(last_name) = last_name {
    iter_match_quotes.push(quote! {
      Some(#last_name) => self.0 = None,
    });
  }
  let trie_tree = crate::trie_tree::build_tree(&trie_nodes);
  let iter_name = syn::Ident::new(&format!("Iter{enum_name}"), Span::call_site());
  let test_mod_name = syn::Ident::new(
    &format!("_enum_test_{}", to_snake_case(&enum_name.to_string())),
    Span::call_site(),
  );
  let parse_match_quote = crate::trie_tree::quote_tree(enum_name, &trie_tree);
  quote! {
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
      #parse_match_quote
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
  }
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
  #[token("xa")]
  Unkown,
}"#;
  let ast: &syn::DeriveInput = &parse_str(input).unwrap();
  let out = inner(ast).unwrap_or_else(|err| err.to_compile_error());
  println!("{}", out.to_string())
}
