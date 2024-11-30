use quote::quote;

#[derive(Debug)]
pub(crate) enum TrieNode<T> {
  Branch { prefix: String, children: Vec<TrieNode<T>> },
  Leaf { suffix: String, ctx: T },
}

pub(crate) fn build_tree<T: Clone>(nodes: &[(T, String)]) -> Vec<TrieNode<T>> {
  fn longest_common_prefix(nodes: &[String]) -> String {
    fn common_prefix(s1: &str, s2: &str) -> String {
      let min_len = s1.len().min(s2.len());
      for i in 0..min_len {
        if s1.as_bytes()[i] != s2.as_bytes()[i] {
          return s1[0..i].to_owned();
        }
      }
      s1[0..min_len].to_owned()
    }
    if nodes.is_empty() {
      return "".to_string();
    }
    let mut prefix = nodes[0].clone();
    for s in nodes.iter().skip(1) {
      prefix = common_prefix(&prefix, s);
      if prefix.is_empty() {
        break;
      }
    }
    prefix.to_string()
  }
  if nodes.is_empty() {
    return vec![];
  }
  if nodes.len() == 1 {
    let (ctx, s) = &nodes[0];
    return vec![TrieNode::Leaf { suffix: s.to_string(), ctx: ctx.clone() }];
  }
  let prefixes: Vec<String> = nodes.iter().map(|(_, s)| s.clone()).collect();
  let prefix = longest_common_prefix(&prefixes);
  if !prefix.is_empty() {
    let suffixes: Vec<(T, String)> = nodes
      .iter()
      .map(|(ctx, s)| (ctx.clone(), s[prefix.len()..].to_owned()))
      .collect();
    let children = build_tree(&suffixes);
    vec![TrieNode::Branch { prefix: prefix.to_string(), children }]
  } else {
    use std::collections::HashMap;
    let mut groups: HashMap<char, Vec<(T, String)>> = HashMap::new();
    for (ctx, s) in nodes {
      let key = s.chars().next().unwrap_or('\0');
      groups
        .entry(key)
        .or_insert_with(Vec::new)
        .push((ctx.clone(), s.to_string()));
    }
    let mut nodes = Vec::new();
    for group in groups.values() {
      let child_nodes = build_tree(group);
      nodes.extend(child_nodes);
    }
    nodes
  }
}
#[cfg(test)]
fn print_tree<T: core::fmt::Display>(node: &TrieNode<T>, indent: usize) {
  let indent_str = "  ".repeat(indent);
  match node {
    TrieNode::Branch { prefix, children } => {
      println!("{}\"{}\"", indent_str, prefix);
      let mut empty_leaf = None;
      for child in children {
        if let TrieNode::Leaf { suffix, ctx } = child {
          if suffix.is_empty() {
            empty_leaf = Some((suffix, ctx));
            continue;
          }
        }
        print_tree(child, indent + 1);
      }
      if let Some((suffix, ctx)) = empty_leaf {
        println!("{}  \"{}\" (ctx: {})", indent_str, suffix, ctx);
      }
    }
    TrieNode::Leaf { suffix, ctx } => {
      println!("{}\"{}\" (ctx: {})", indent_str, suffix, ctx);
    }
  }
}
pub(crate) fn quote_node(
  node: &TrieNode<proc_macro2::TokenStream>,
) -> proc_macro2::TokenStream {
  match node {
    TrieNode::Branch { prefix, children } => {
      let mut arms = Vec::new();
      let mut empty_leaf = None;
      for child in children {
        if let TrieNode::Leaf { suffix, ctx } = child {
          if suffix.is_empty() {
            empty_leaf = Some(ctx);
            continue;
          }
        }
        arms.push(quote_node(child));
      }
      if let Some(ctx) = empty_leaf {
        arms.push(quote! {
          map(take(0u8),|_| #ctx),
        });
      }
      quote! {
        preceded(tag(#prefix),
          alt((
            #(#arms)*
          ))
        ),
      }
    }
    TrieNode::Leaf { suffix, ctx } => {
      if suffix.len() == 1 {
        let c = suffix.chars().next().unwrap();
        quote! {
          map(char(#c), |_| #ctx),
        }
      } else {
        quote! {
          map(tag(#suffix), |_| #ctx),
        }
      }
    }
  }
}

pub(crate) fn quote_tree(
  ident: &syn::Ident,
  tree: &[crate::trie_tree::TrieNode<proc_macro2::TokenStream>],
) -> proc_macro2::TokenStream {
  let parse_match_quotes: Vec<_> =
    tree.iter().map(crate::trie_tree::quote_node).collect();
  quote! {
    use nom::{
      error::Error, IResult,
      branch::alt, bytes::complete::{tag, take}, character::complete::char, combinator::map,
      sequence::{delimited, preceded},
    };
    impl crate::ast::NomParseTerm for #ident {
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
  }
}
#[test]
fn test() {
  let input = vec![
    (1, "test_scan_in_inverted".to_owned()),
    (2, "test_scan_in".to_owned()),
    (3, "test_scan_out_inverted".to_owned()),
    (4, "test_scan_out".to_owned()),
    (5, "test_scan_enable_inverted".to_owned()),
    (6, "test_scan_enable".to_owned()),
    (7, "test_scan_clock_a".to_owned()),
    (8, "test_scan_clock_b".to_owned()),
    (9, "test_scan_clock".to_owned()),
    (10, "test_clock".to_owned()),
    (11, "te".to_owned()),
  ];
  let tree = build_tree(&input);
  for node in &tree {
    print_tree(node, 0);
  }
}
