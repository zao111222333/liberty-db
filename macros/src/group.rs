use crate::attribute::*;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

fn group_field_fn(
  field_name: &Ident,
  arrti_type: &AttriType,
) -> syn::Result<(
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
)> {
  let s_field_name = field_name.to_string();
  let attri_comment: _;
  let write_field: _;
  let parser_arm: _;
  match arrti_type {
    AttriType::Simple(SimpleType::Option) => {
      attri_comment = quote! {
        pub #field_name: crate::ast::AttriComment,
      };
      write_field = quote! {
        if let Some(simple) = &self.#field_name {
          crate::ast::Format::liberty(&self.comments().#field_name, "", f)?;
          crate::ast::SimpleAttri::fmt_liberty(simple, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let simple_res: _;
        (input,simple_res) = <_ as crate::ast::SimpleAttri>::nom_parse(input,line_num)?;
        match simple_res {
          Ok(simple) => {
            res.#field_name=Some(simple);
          },
          Err((e,undefined)) => {
            println!("Line={}; Key={}; Value={:?}; Err={}",line_num,key,undefined,e);
            res.undefined_list_mut().push((key.to_owned(), undefined));
          },
        }
      };
    }
    AttriType::Simple(SimpleType::Default) => {
      attri_comment = quote! {
        pub #field_name: crate::ast::AttriComment,
      };
      write_field = quote! {
        crate::ast::Format::liberty(&self.comments().#field_name, "", f)?;
        crate::ast::SimpleAttri::fmt_liberty(&self.#field_name, #s_field_name, f)?;
      };
      parser_arm = quote! {
        let simple_res: _;
        (input,simple_res) = <_ as crate::ast::SimpleAttri>::nom_parse(input,line_num)?;
        match simple_res {
          Ok(simple) => {
            res.#field_name=simple;
          },
          Err((e,undefined)) => {
            println!("Line={}; Key={}; Value={:?}; Err={}",line_num,key,undefined,e);
            res.undefined_list_mut().push((key.to_owned(), undefined));
          },
        }
      };
    }
    AttriType::Complex(ComplexType::Default) => {
      attri_comment = quote! {
        pub #field_name: crate::ast::AttriComment,
      };
      write_field = quote! {
        crate::ast::Format::liberty(&self.comments().#field_name, "", f)?;
        crate::ast::ComplexAttri::fmt_liberty(&self.#field_name, #s_field_name, f)?;
      };
      parser_arm = quote! {
        let complex_res: _;
        (input,complex_res) = <_ as crate::ast::ComplexAttri>::nom_parse(input,line_num)?;
        match complex_res {
          Ok(complex) => res.#field_name=complex,
          Err(e) => {
            println!("Line={}; Key={}; Err={}",line_num,key,e);
          },
        }
      };
    }
    AttriType::Complex(ComplexType::Option) => {
      attri_comment = quote! {
        pub #field_name: crate::ast::AttriComment,
      };
      write_field = quote! {
        if let Some(complex) = &self.#field_name {
          crate::ast::Format::liberty(&self.comments().#field_name, "", f)?;
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let complex_res: _;
        (input,complex_res) = <_ as crate::ast::ComplexAttri>::nom_parse(input,line_num)?;
        match complex_res {
          Ok(complex) => res.#field_name=Some(complex),
          Err(e) => {
            println!("Line={}; Key={}; Err={}",line_num,key,e);
          },
        }
      };
    }
    AttriType::Group(GroupType::Vec) => {
      attri_comment = quote! {};
      write_field = quote! {
        for group in self.#field_name.iter(){
          <crate::ast::AttriComment as crate::ast::Format>::liberty(group.comment(), "", f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let group_res: _;
        (input,group_res) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
        match group_res{
          Ok(group) => {
            res.#field_name.push(group);
          },
          Err(e) => {
            println!("Line={}, error={}",line_num,e);
          },
        }
        let n: usize;
        (input,n) = crate::ast::parser::comment_space_newline(input)?;
        *line_num+=n;
      };
    }
    AttriType::Group(GroupType::Set) => {
      attri_comment = quote! {};
      write_field = quote! {
        for group in self.#field_name.iter(){
          <crate::ast::AttriComment as crate::ast::Format>::liberty(group.comment(), "", f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let group_res: _;
        (input,group_res) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
        match group_res{
          Ok(group) => {
            if let Some(_) = res.#field_name.replace(
              group,
            ){
              let e = crate::ast::IdError::RepeatIdx;
              println!("Line={}, error={}",line_num,e);
            }
          },
          Err(e) => {
            println!("Line={}, error={}",line_num,e);
          },
        }
        let n: usize;
        (input,n) = crate::ast::parser::comment_space_newline(input)?;
        *line_num+=n;
      };
    }
    AttriType::Group(GroupType::Option) => {
      attri_comment = quote! {};
      write_field = quote! {
        if let Some(group) = &self.#field_name {
          <crate::ast::AttriComment as crate::ast::Format>::liberty(group.comment(), "", f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let group_res: _;
        (input,group_res) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
        match group_res{
          Ok(group) => {
            if let Some(old) = res.#field_name{
              let e = crate::ast::IdError::RepeatIdx;
              println!("Line={}, error={}",line_num,e);
            }
            res.#field_name = Some(group);
          },
          Err(e) => {
            println!("Line={}, error={}",line_num,e);
          },
        }
        let n: usize;
        (input,n) = crate::ast::parser::comment_space_newline(input)?;
        *line_num+=n;
      };
    }
  }
  Ok((
    attri_comment,
    write_field,
    quote!(
      #s_field_name => {
        #parser_arm
      },
    ),
  ))
}

pub(crate) fn inner(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let name = &ast.ident;
  let st = match &ast.data {
    Data::Struct(s) => s,
    _ => {
      return Err(syn::Error::new(Span::call_site(), "This macro only supports struct."))
    }
  };

  if let Fields::Named(named) = &st.fields {
    let fields = &named.named;
    let (attri_type_map, name_name, undefined_name, comments_name) =
      parse_fields_type(fields)?;
    let mut attri_comments = quote! {};
    let mut parser_arms = quote! {};
    let mut write_fields = quote! {};

    for field in fields.into_iter() {
      if let Some(field_name) = &field.ident {
        match attri_type_map.get(field_name) {
          None => {}
          Some(arrti_type) => {
            let (attri_comment, write_field, parser_arm) =
              group_field_fn(field_name, arrti_type)?;
            attri_comments = quote! {
              #attri_comments
              #attri_comment
            };
            parser_arms = quote! {
              #parser_arms
              #parser_arm
            };
            write_fields = quote! {
              #write_fields
              #write_field
            };
          }
        }
      } else {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          format!("Can not find field ident!"),
        ));
      }
    }
    let change_id_return = if let Some(name_name) = name_name {
      quote! {
        match crate::ast::NameAttri::parse(title){
          Ok(name) => {
            res.#name_name = name;
            return Ok((input,Ok(res)));
          },
          Err(e) => {
            return Ok((input,Err(e)));
          },
        }
      }
    } else {
      quote! {return Ok((input, Ok(res)));}
    };
    let write_title = if let Some(name_name) = name_name {
      quote! {
        write!(f,"\n{} (",key)?;
        crate::ast::NameAttri::fmt_liberty(&self.#name_name, f)?;
        write!(f,") {{")?;
      }
    } else {
      quote! {
        write!(f,"\n{} () {{",key)?;
      }
    };
    let comments_ident = Ident::new(&format!("{}Comments", name), Span::call_site());
    let comments_self = Ident::new("_self", Span::call_site());
    let comments_undefined = Ident::new("_undefined", Span::call_site());
    let impl_group = quote! {
      #[doc(hidden)]
      #[derive(Default,Debug,Clone)]
      pub struct #comments_ident{
        #comments_self: crate::ast::AttriComment,
        #comments_undefined: crate::ast::AttriComment,
        #attri_comments
      }
      impl crate::ast::GroupAttri for #name {
        type Comments=#comments_ident;
        #[inline]
        fn comment(&self) -> &crate::ast::AttriComment{
          &self.#comments_name.#comments_self
        }
        #[inline]
        fn comment_mut(&mut self) -> &mut crate::ast::AttriComment{
          &mut self.#comments_name.#comments_self
        }
        #[inline]
        fn comments(&self) -> &Self::Comments{
          &self.#comments_name
        }
        #[inline]
        fn comments_mut(&mut self) -> &mut Self::Comments{
          &mut self.#comments_name
        }
        #[inline]
        fn undefined_list(&self)-> &crate::ast::AttributeList{
          &self.#undefined_name
        }
        #[inline]
        fn undefined_list_mut(&mut self)-> &mut crate::ast::AttributeList{
          &mut self.#undefined_name
        }
        fn fmt_liberty<T: std::fmt::Write>(&self, key: &str, f: &mut crate::ast::CodeFormatter<'_, T>) -> std::fmt::Result {
          use std::fmt::Write;
          use itertools::Itertools;
          #write_title
          f.indent(1);
          #write_fields
          if !self.undefined_list().is_empty(){
            <crate::ast::AttriComment as crate::ast::Format>::liberty(&self.#comments_name.#comments_undefined, "", f)?;
            crate::ast::liberty_attr_list(&self.undefined_list(),f)?;
          }
          f.dedent(1);
          write!(f, "\n}}")
        }
        fn nom_parse<'a>(
          i: &'a str, line_num: &mut usize
        ) -> nom::IResult<&'a str, Result<Self,crate::ast::IdError>, nom::error::Error<&'a str>> {
          let (mut input,title) = crate::ast::parser::title(i,line_num)?;
          let mut res = Self::default();
          res.comments_mut().#comments_undefined.push("Undefined attributes from here".to_string());
          loop {
            match crate::ast::parser::key(input){
              Err(nom::Err::Error(_)) => {
                (input,_) = crate::ast::parser::end_group(input)?;
                #change_id_return
              },
              Err(e) => return Err(e),
              Ok((_input,key)) => {
                input = _input;
                match key {
                  #parser_arms
                  _ => {
                    let undefined: crate::ast::AttriValue;
                    (input,undefined) = crate::ast::parser::undefine(input,line_num)?;
                    res.undefined_list_mut().push((key.to_owned(), undefined));
                    let n: usize;
                    (input,n) = crate::ast::parser::comment_space_newline(input)?;
                    *line_num+=n;
                  },
                }
              }
            }
          }
        }
      }
    };
    Ok(quote! {
      // #impl_hashed_group
      #impl_group
    })
  } else {
    Err(syn::Error::new(Span::call_site(), format!("Can not find NamedField")))
  }
}

#[test]
fn main() {
  use syn::parse_str;
  let input = r#"
  #[derive(liberty_macros::Group)]
  struct Timing {
    #[liberty(undefined)]
    _undefined: AttributeList,
    #[liberty(comments)]
    _comments: GroupComments<Self>,
    #[liberty(complex)]
    values: Vec<f64>,
    #[liberty(simple(type = Option))]
    t1: Option<TimingType>,
    #[liberty(simple(type = Option))]
    t2: Option<TimingType>,
  }"#;
  let ast: &syn::DeriveInput = &parse_str(input).unwrap();
  let out = inner(&ast).unwrap_or_else(|err| err.to_compile_error().into());
  println!("{}", out)
}
