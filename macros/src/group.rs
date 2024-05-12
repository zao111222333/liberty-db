use crate::attribute::*;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{parse_quote, Data, DeriveInput, Field, Fields, Token};

fn group_field_fn(
  field_name: &Ident,
  arrti_type: &AttriType,
  undefined_name: &Ident,
  comments_name: &Ident,
  comments_self: &Ident,
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
          crate::ast::Format::liberty(&self.#comments_name.#field_name, "", f)?;
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
            res.#undefined_name.push((key.to_owned(), undefined));
          },
        }
      };
    }
    AttriType::Simple(SimpleType::Default) => {
      attri_comment = quote! {
        pub #field_name: crate::ast::AttriComment,
      };
      write_field = quote! {
        crate::ast::Format::liberty(&self.#comments_name.#field_name, "", f)?;
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
            res.#undefined_name.push((key.to_owned(), undefined));
          },
        }
      };
    }
    AttriType::Complex(ComplexType::Default) => {
      attri_comment = quote! {
        pub #field_name: crate::ast::AttriComment,
      };
      write_field = quote! {
        crate::ast::Format::liberty(&self.#comments_name.#field_name, "", f)?;
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
          crate::ast::Format::liberty(&self.#comments_name.#field_name, "", f)?;
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
    AttriType::Complex(ComplexType::Vec) => {
      attri_comment = quote! {};
      write_field = quote! {
        for complex in self.#field_name.iter(){
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let complex_res: _;
        (input,complex_res) = <_ as crate::ast::ComplexAttri>::nom_parse(input, line_num)?;
        match complex_res{
          Ok(complex) => {
            res.#field_name.push(complex);
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
    AttriType::Complex(ComplexType::Set) => {
      attri_comment = quote! {};
      write_field = quote! {
        for complex in self.#field_name.iter_sort(){
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let complex_res: _;
        (input,complex_res) = <_ as crate::ast::ComplexAttri>::nom_parse(input, line_num)?;
        match complex_res{
          Ok(complex) => {
            if let Some(_) = res.#field_name.replace(
              complex,
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
    AttriType::Group(GroupType::Vec) => {
      attri_comment = quote! {};
      write_field = quote! {
        for group in self.#field_name.iter(){
          <crate::ast::AttriComment as crate::ast::Format>::liberty(&group.#comments_name.#comments_self, "", f)?;
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
        for group in self.#field_name.iter_sort(){
          <crate::ast::AttriComment as crate::ast::Format>::liberty(&group.#comments_name.#comments_self, "", f)?;
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
          <crate::ast::AttriComment as crate::ast::Format>::liberty(&group.#comments_name.#comments_self, "", f)?;
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

pub(crate) fn inner(
  ast: &DeriveInput,
  link: bool,
) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &ast.ident;
  let st = match &ast.data {
    Data::Struct(s) => s,
    _ => {
      return Err(syn::Error::new(Span::call_site(), "This macro only supports struct."))
    }
  };

  if let Fields::Named(named) = &st.fields {
    let fields = &named.named;
    let (attri_type_map, name_vec, undefined_name, comments_name) =
      parse_fields_type(fields)?;
    let mut attri_comments = quote! {};
    let mut parser_arms = quote! {};
    let mut write_fields = quote! {};
    let comments_self = Ident::new("name", Span::call_site());
    for field in fields.into_iter() {
      if let Some(field_name) = &field.ident {
        match attri_type_map.get(field_name) {
          None => {}
          Some(arrti_type) => {
            let (attri_comment, write_field, parser_arm) = group_field_fn(
              field_name,
              arrti_type,
              undefined_name,
              comments_name,
              &comments_self,
            )?;
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
    let link_self = if link {
      quote! {<Self as crate::ast::GroupFn>::post_process(&mut res);}
    } else {
      quote! {}
    };
    let (change_id_return, write_title) = if name_vec.len() == 0 {
      (
        quote! {return Ok((input, Ok(res)));},
        quote! {
          write!(f,"\n{} () {{",key)?;
        },
      )
    } else {
      (
        quote! {
          match <Self as crate::ast::NamedGroup>::parse(title){
            Ok(name) => {
              res.set_name(name);
              return Ok((input,Ok(res)));
            },
            Err(e) => {
              return Ok((input,Err(e)));
            },
          }
        },
        quote! {
          write!(f,"\n{} (",key)?;
          crate::ast::NamedGroup::fmt_liberty(self, f)?;
          write!(f,") {{")?;
        },
      )
    };
    let comments_ident = Ident::new(&format!("{}Comments", ident), Span::call_site());
    let comments_undefined_bgn = Ident::new("_undefined_bgn", Span::call_site());
    let comments_undefined_end = Ident::new("_undefined_end", Span::call_site());
    let (name_ident, name_func, name_sturct, named_group_impl) = match name_vec.len() {
      0 => (
        quote!(()),
        quote! {
          #[inline]
          fn name(&self) -> Self::Name{()}
          #[inline]
          fn set_name(&mut self, name: Self::Name){}
        },
        quote!(),
        quote!(),
      ),
      1 => {
        let t = &name_vec[0].ty;
        let i = name_vec[0].ident.clone().expect("name has no ident!");
        (
          quote!(#t),
          quote! {
            #[inline]
            fn name(&self) -> Self::Name{
              self.#i.clone()
            }
            #[inline]
            fn set_name(&mut self, name: Self::Name){
              self.#i = name;
            }
          },
          quote!(),
          quote! {
            impl crate::ast::NamedGroup for #ident {
              #[inline]
              fn parse(v: Vec<String>) -> Result<Self::Name, crate::ast::IdError>{
                <Self::Name as crate::ast::NameAttri>::parse(v)
              }
              #[inline]
              fn name2vec(name: Self::Name) -> Vec<String>{
                <Self::Name as crate::ast::NameAttri>::to_vec(name)
              }
            }
          },
        )
      }
      _ => {
        let i = Ident::new(&format!("{}Name", ident), Span::call_site());
        let mut s: DeriveInput = parse_quote! {
          #[doc(hidden)]
          #[derive(Debug,Clone)]
          pub struct #i{
          }
        };
        let s_fileds = fields_of_input(&mut s);
        let mut _name = quote!();
        let mut _set_name = quote!();
        for f in name_vec.into_iter() {
          let mut f = f.clone();
          let i = f.ident.clone().expect("name has no ident!");
          f.attrs.clear();
          s_fileds.push(f);
          _name = quote!(#_name
            #i:self.#i.clone(),
          );
          _set_name = quote!(#_set_name
            self.#i = name.#i;
          );
        }
        (
          quote!(#i),
          quote! {
            #[inline]
              fn name(&self) -> Self::Name{
                Self::Name{
                  #_name
                }
              }
              #[inline]
              fn set_name(&mut self, name: Self::Name){
                #_set_name
              }
          },
          quote!(#s),
          quote!(),
        )
      }
    };
    let impl_group = quote! {
      #named_group_impl
      #name_sturct
      #[doc(hidden)]
      #[derive(Default,Debug,Clone)]
      pub struct #comments_ident{
        pub #comments_self: crate::ast::AttriComment,
        pub #comments_undefined_bgn: crate::ast::AttriComment,
        pub #comments_undefined_end: crate::ast::AttriComment,
        #attri_comments
      }
      impl crate::ast::GroupAttri for #ident {
        type Name=#name_ident;
        type Comments=#comments_ident;
        #name_func
        fn fmt_liberty<T: std::fmt::Write>(&self, key: &str, f: &mut crate::ast::CodeFormatter<'_, T>) -> std::fmt::Result {
          use std::fmt::Write;
          use itertools::Itertools;
          #write_title
          f.indent(1);
          if !self.#undefined_name.is_empty(){
            <crate::ast::AttriComment as crate::ast::Format>::liberty(&self.#comments_name.#comments_undefined_bgn, "", f)?;
            crate::ast::liberty_attr_list(&self.#undefined_name,f)?;
            <crate::ast::AttriComment as crate::ast::Format>::liberty(&self.#comments_name.#comments_undefined_end, "", f)?;
          }
          #write_fields
          f.dedent(1);
          write!(f, "\n}}")
        }
        fn nom_parse<'a>(
          i: &'a str, line_num: &mut usize
        ) -> nom::IResult<&'a str, Result<Self,crate::ast::IdError>, nom::error::Error<&'a str>> {
          let (mut input,title) = crate::ast::parser::title(i,line_num)?;
          let mut res = Self::default();
          res.#comments_name.#comments_undefined_bgn.push("Undefined attributes from here".to_string());
          res.#comments_name.#comments_undefined_end.push("Undefined attributes end here".to_string());
          loop {
            match crate::ast::parser::key(input){
              Err(nom::Err::Error(_)) => {
                (input,_) = crate::ast::parser::end_group(input)?;
                #link_self
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
                    res.#undefined_name.push((key.to_owned(), undefined));
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
      #impl_group
    })
  } else {
    Err(syn::Error::new(Span::call_site(), format!("Can not find NamedField")))
  }
}
type Punctuated = syn::punctuated::Punctuated<Field, Token![,]>;
fn fields_of_input(input: &mut DeriveInput) -> &mut Punctuated {
  match &mut input.data {
    Data::Struct(data) => match &mut data.fields {
      Fields::Named(fields) => &mut fields.named,
      Fields::Unnamed(fields) => &mut fields.unnamed,
      Fields::Unit => unreachable!(),
    },
    Data::Enum(_) | Data::Union(_) => unreachable!(),
  }
}

#[test]
fn main() {
  use syn::parse_str;
  let input = r#"
  #[derive(liberty_macros::Group)]
  struct Timing {
    #[liberty(undefined)]
    pub undefined: AttributeList,
    #[liberty(comments)]
    pub comments: GroupComments<Self>,
    #[liberty(complex)]
    values: Vec<f64>,
    #[liberty(simple(type = Option))]
    t1: Option<TimingType>,
    #[liberty(simple(type = Option))]
    t2: Option<TimingType>,
  }"#;
  let ast: &syn::DeriveInput = &parse_str(input).unwrap();
  let out = inner(&ast, false).unwrap_or_else(|err| err.to_compile_error().into());
  println!("{}", out)
}
