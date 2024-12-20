use crate::attribute::*;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, Fields};

fn group_field_fn(
  field_name: &Ident,
  default: Option<&proc_macro2::TokenStream>,
  arrti_type: &AttriType,
  attributes_name: &Ident,
  comments_name: &Ident,
  hashmatch: bool,
) -> syn::Result<(
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
)> {
  let s_field_name = field_name.to_string();
  let comment_fn_name =
    Ident::new(&format!("comments_{s_field_name}"), Span::call_site());
  let comment_this_fn = Ident::new("comments_this", Span::call_site());
  let comment_fn_entry =
    Ident::new(&format!("comments_{s_field_name}_entry"), Span::call_site());
  let mut comment_fn = quote! {
    #[inline]
    pub fn #comment_fn_name(&self)-> Option<&String> {
      self.#comments_name.0.get(&hashmatch::hash_arm!(#s_field_name))
    }
    #[inline]
    pub fn #comment_fn_entry<'a>(&'a mut self)-> std::collections::hash_map::Entry<'a, u64, String> {
      self.#comments_name.0.entry(hashmatch::hash_arm!(#s_field_name))
    }
  };
  let write_field: proc_macro2::TokenStream;
  let mut parser_init = if let Some(default) = default {
    quote! { let mut #field_name = #default; }
  } else {
    quote! { let mut #field_name = Default::default(); }
  };

  let parser_arm: proc_macro2::TokenStream;
  let mut parser_post = quote! {#field_name,};
  match arrti_type {
    AttriType::Simple(SimpleType::Option) => {
      write_field = quote! {
        if let Some(simple) = &self.#field_name {
          crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
          crate::ast::SimpleAttri::fmt_liberty(simple, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let (new_input,simple_res) = crate::ast::SimpleAttri::nom_parse(input, scope)?;
        input = new_input;
        match simple_res {
          Ok(simple) => {
            #field_name=Some(simple);
          },
          Err(undefined) => {
            log::error!("Line={}; Key={}; Value={:?}",scope.line_num,key,undefined);
            crate::ast::attributs_set_undefined_simple(&mut #attributes_name, key, undefined);
          },
        }
      };
    }
    AttriType::Simple(SimpleType::Default) => {
      write_field = quote! {
        crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
        crate::ast::SimpleAttri::fmt_liberty(&self.#field_name, #s_field_name, f)?;
      };
      parser_arm = quote! {
        let (new_input,simple_res) = crate::ast::SimpleAttri::nom_parse(input, scope)?;
        input = new_input;
        match simple_res {
          Ok(simple) => {
            #field_name=simple;
          },
          Err(undefined) => {
            log::error!("Line={}; Key={}; Value={:?}",scope.line_num,key,undefined);
            crate::ast::attributs_set_undefined_simple(&mut #attributes_name, key, undefined);
          },
        }
      };
    }
    AttriType::Complex(ComplexType::Default) => {
      write_field = quote! {
        crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
        crate::ast::ComplexAttri::fmt_liberty(&self.#field_name, #s_field_name, f)?;
      };
      parser_arm = quote! {
        let (new_input,complex_res) = crate::ast::ComplexAttri::nom_parse(input, scope)?;
        input = new_input;
        match complex_res {
          Ok(complex) => #field_name=complex,
          Err((e,undefined)) => {
            log::error!("Line={}; Key={}; Value={:?}; Err={}",scope.line_num,key,undefined,e);
            crate::ast::attributs_set_undefined_complex(&mut #attributes_name, key, undefined);
          },
        }
      };
    }
    AttriType::Complex(ComplexType::Option) => {
      write_field = quote! {
        if let Some(complex) = &self.#field_name {
          crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let (new_input,complex_res) = crate::ast::ComplexAttri::nom_parse(input, scope)?;
        input = new_input;
        match complex_res {
          Ok(complex) => #field_name=Some(complex),
          Err((e,undefined)) => {
            log::error!("Line={}; Key={}; Value={:?}; Err={}",scope.line_num,key,undefined,e);
            crate::ast::attributs_set_undefined_complex(&mut #attributes_name, key, undefined);
          },
        }
      };
    }
    AttriType::Complex(ComplexType::Vec) => {
      comment_fn = quote! {};
      write_field = quote! {
        for complex in self.#field_name.iter(){
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let (new_input,complex_res) = crate::ast::ComplexAttri::nom_parse(input, scope)?;
        input = new_input;
        match complex_res{
          Ok(complex) => {
            #field_name.push(complex);
          },
          Err((e,undefined)) => {
            log::error!("Line={}; Key={}; Value={:?}; Err={}",scope.line_num,key,undefined,e);
            crate::ast::attributs_set_undefined_complex(&mut #attributes_name, key, undefined);
          },
        }
        let n: usize;
        (input,n) = crate::ast::parser::comment_space_newline(input)?;
        scope.line_num += n;
      };
    }
    AttriType::Complex(ComplexType::Set) => {
      comment_fn = quote! {};
      write_field = quote! {
        for complex in self.#field_name.iter_sort(){
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      cfg_if::cfg_if! {
        if #[cfg(feature = "vec2map")] {
          // Vec then into HashMap
          parser_init = quote! {
            let mut #field_name = Vec::new();
          };
          let insert = quote! {
            #field_name.push(complex);
          };
          parser_post = quote! {
            #field_name: #field_name.into_iter().collect(),
          };
        } else {
          // Directly into HashMap
          let insert = quote! {
            if let Some(_) = #field_name.replace(
              complex,
            ){
              let e = crate::ast::IdError::RepeatAttri;
              log::error!("Line={}, error={}",scope.line_num,e);
            }
          };
        }
      }
      parser_arm = quote! {
        let (new_input,complex_res) = crate::ast::ComplexAttri::nom_parse(input, scope)?;
        input = new_input;
        match complex_res {
          Ok(complex) => {
            #insert
          },
          Err((e,undefined)) => {
            log::error!("Line={}; Key={}; Value={:?}; Err={}",scope.line_num,key,undefined,e);
            crate::ast::attributs_set_undefined_complex(&mut #attributes_name, key, undefined);
          },
        }
        let n: usize;
        (input,n) = crate::ast::parser::comment_space_newline(input)?;
        scope.line_num += n;
      };
    }
    AttriType::Group(GroupType::Vec) => {
      comment_fn = quote! {};
      write_field = quote! {
        for group in self.#field_name.iter(){
          crate::ast::fmt_comment_liberty(group.#comment_this_fn(), f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let (new_input,group_res) = crate::ast::GroupAttri::nom_parse(input, key, scope)?;
        input = new_input;
        match group_res{
          Ok(group) => {
            #field_name.push(group);
          },
          Err(e) => {
            log::error!("Line={}, error={}",scope.line_num,e);
          },
        }
        let n: usize;
        (input,n) = crate::ast::parser::comment_space_newline(input)?;
        scope.line_num += n;
      };
    }
    AttriType::Group(GroupType::Set) => {
      comment_fn = quote! {};
      write_field = quote! {
        for group in self.#field_name.iter_sort(){
          crate::ast::fmt_comment_liberty(group.#comment_this_fn(), f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      cfg_if::cfg_if! {
        if #[cfg(feature = "vec2map")] {
          // Vec then into HashMap
          parser_init = quote! {
            let mut #field_name = Vec::new();
          };
          let insert = quote! {
            #field_name.push(group);
          };
          parser_post = quote! {
            #field_name: #field_name.into_iter().collect(),
          };
        } else {
          // Directly into HashMap
          let insert = quote! {
            if let Some(_) = #field_name.replace(
              group,
            ){
              let e = crate::ast::IdError::RepeatAttri;
              log::error!("Line={}, error={}",scope.line_num,e);
            }
          };
        }
      }
      parser_arm = quote! {
        let (new_input,group_res) = crate::ast::GroupAttri::nom_parse(input, key, scope)?;
        input = new_input;
        match group_res{
          Ok(group) => {
            #insert
          },
          Err(e) => {
            log::error!("Line={}, error={}",scope.line_num,e);
          },
        }
        let n: usize;
        (input,n) = crate::ast::parser::comment_space_newline(input)?;
        scope.line_num += n;
      };
    }
    AttriType::Group(GroupType::Option) => {
      comment_fn = quote! {};
      write_field = quote! {
        if let Some(group) = &self.#field_name {
          crate::ast::fmt_comment_liberty(group.#comment_this_fn(), f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        let (new_input,group_res) = crate::ast::GroupAttri::nom_parse(input, key, scope)?;
        input = new_input;
        match group_res{
          Ok(group) => {
            if let Some(old) = #field_name{
              let e = crate::ast::IdError::RepeatAttri;
              log::error!("Line={}, error={}",scope.line_num,e);
            }
            #field_name = Some(group);
          },
          Err(e) => {
            log::error!("Line={}, error={}",scope.line_num,e);
          },
        }
        let n: usize;
        (input,n) = crate::ast::parser::comment_space_newline(input)?;
        scope.line_num += n;
      };
    }
  }
  Ok((
    comment_fn,
    write_field,
    parser_init,
    if hashmatch {
      quote!(
        hashmatch::hash_arm!(#s_field_name) => {
          #parser_arm
        },
      )
    } else {
      quote!(
        #s_field_name => {
          #parser_arm
        },
      )
    },
    parser_post,
  ))
}

pub(crate) fn inner(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &ast.ident;
  let st = match &ast.data {
    Data::Struct(s) => s,
    _ => {
      return Err(syn::Error::new(Span::call_site(), "This macro only supports struct."))
    }
  };

  if let Fields::Named(named) = &st.fields {
    let fields = &named.named;
    let (attri_type_map, default_map, name_vec, attributes_name, comments_name) =
      parse_fields_type(fields)?;
    let this_str = "__this__";
    let mut comment_fns = quote! {
      #[inline]
      pub fn comments_this(&self)-> Option<&String> {
        self.#comments_name.0.get(&hashmatch::hash_arm!(#this_str))
      }
      #[inline]
      pub fn comments_this_entry<'a>(&'a mut self)-> std::collections::hash_map::Entry<'a, u64, String> {
        self.#comments_name.0.entry(hashmatch::hash_arm!(#this_str))
      }
    };

    let mut parser_inits = quote! {};
    parser_inits = if let Some(default) = default_map.get(attributes_name) {
      quote! {
        let mut #attributes_name = #default;
        #parser_inits
      }
    } else {
      quote! {
        let mut #attributes_name = Default::default();
        #parser_inits
      }
    };
    let mut parser_arms = quote! {};
    let mut parser_posts =
      name_vec.iter().fold(quote! {#attributes_name, }, |old, name| {
        let i = name.ident.as_ref().unwrap();
        if let Some(defalut) = default_map.get(i) {
          quote! {#old #i: #defalut,}
        } else {
          old
        }
      });
    let defalut_impl = fields.iter().fold(quote! {}, |old, field| {
      let i = field.ident.as_ref().unwrap();
      if let Some(defalut) = default_map.get(i) {
        quote! {#old #i: #defalut,}
      } else {
        quote! {#old #i: Default::default(),}
      }
    });
    let mut write_simple_complex = quote! {};
    let mut write_group = quote! {};
    let mut field_name_arrti_type_old_pos = Vec::new();
    for field in fields.into_iter() {
      if let Some(field_name) = &field.ident {
        if let Some((arrti_type, old_pos)) = attri_type_map.get(field_name) {
          field_name_arrti_type_old_pos.push((field_name, arrti_type, old_pos));
        }
      } else {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          "Can not find field ident!".to_string(),
        ));
      }
    }
    field_name_arrti_type_old_pos.sort_by(|(_, _, a), (_, _, b)| match (a, b) {
      (None, None) => std::cmp::Ordering::Equal,
      (None, Some(_)) => std::cmp::Ordering::Greater,
      (Some(_), None) => std::cmp::Ordering::Less,
      (Some(a), Some(b)) => a.cmp(b),
    });
    let hashmatch = if field_name_arrti_type_old_pos.len() >= 40 {
      cfg_if::cfg_if! {
        if #[cfg(feature = "hashmatch")] {
          true
        }else{
          false
        }
      }
    } else {
      false
    };
    for (field_name, arrti_type, _) in field_name_arrti_type_old_pos {
      let (comment_fn, write_field, parser_init, parser_arm, parser_post) =
        group_field_fn(
          field_name,
          default_map.get(field_name),
          arrti_type,
          attributes_name,
          comments_name,
          hashmatch,
        )?;
      comment_fns = quote! {
        #comment_fns
        #comment_fn
      };
      parser_inits = quote! {
        #parser_inits
        #parser_init
      };
      parser_arms = quote! {
        #parser_arms
        #parser_arm
      };
      parser_posts = quote! {
        #parser_posts
        #parser_post
      };
      match arrti_type {
        AttriType::Simple(_) | AttriType::Complex(_) => {
          write_simple_complex = quote! {
            #write_simple_complex
            #write_field
          }
        }
        AttriType::Group(_) => {
          write_group = quote! {
            #write_group
            #write_field
          }
        }
      }
    }
    let (change_id_return, write_title) = if name_vec.is_empty() {
      (
        quote! {return Ok((input, Ok(res)));},
        quote! {
          write!(f,"\n{indent}{key} () {{")?;
        },
      )
    } else {
      (
        quote! {
          return Ok((input,
            match <Self as crate::ast::NamedGroup>::parse_set_name(&mut res, title){
              Ok(_) => {
                Ok(res)
              },
              Err(e) => {
                Err(e)
              },
            }
          ));
        },
        quote! {
          write!(f,"\n{indent}{key} (")?;
          crate::ast::NamedGroup::fmt_name(self, f)?;
          write!(f,") {{")?;
        },
      )
    };
    let named_group_impl = match name_vec.len() {
      1 => {
        let t = &name_vec[0].ty;
        let i = name_vec[0].ident.clone().expect("name has no ident!");
        quote! {
          #[doc(hidden)]
          impl crate::ast::NamedGroup for #ident {
            #[inline]
            fn parse_set_name(&mut self, v: Vec<&str>) -> Result<(), crate::ast::IdError> {
              <#t as crate::ast::NameAttri>::parse(v).map(|name| {self.#i = name;})
            }
            #[inline]
            fn fmt_name<T: core::fmt::Write, I: crate::ast::Indentation>(
              &self,
              f: &mut crate::ast::CodeFormatter<'_, T, I>,
            ) -> core::fmt::Result
            {
              <#t as crate::ast::NameAttri>::fmt_self(&self.#i, f)
            }
          }
        }
      }
      _ => quote!(),
    };
    let key_id = if hashmatch { quote!(hashmatch::hash_str(key)) } else { quote!(key) };

    let impl_group = quote! {
      #[doc(hidden)]
      impl Default for #ident {
        #[inline]
        fn default() -> Self {
          Self {
            #defalut_impl
          }
        }
      }
      #named_group_impl
      impl #ident {
        #comment_fns
      }
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl crate::ast::Group for #ident {}
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl crate::ast::GroupAttri for #ident {
        fn fmt_liberty<T: core::fmt::Write, I: crate::ast::Indentation>(&self, key: &str, f: &mut crate::ast::CodeFormatter<'_, T, I>) -> core::fmt::Result {
          use core::fmt::Write;
          use itertools::Itertools;
          let indent = f.indentation();
          #write_title
          f.indent(1);
          #write_simple_complex
          if !self.#attributes_name.is_empty(){
            crate::ast::attributs_fmt_liberty(&self.#attributes_name,f)?;
          }
          #write_group
          f.dedent(1);
          write!(f, "\n{indent}}}")
        }
        fn nom_parse<'a>(
          i: &'a str,
          group_name: &str,
          scope: &mut crate::ast::ParseScope,
        ) -> nom::IResult<&'a str, Result<Self,crate::ast::IdError>, nom::error::Error<&'a str>> {
          let (mut input,title) = crate::ast::parser::title(i, &mut scope.line_num)?;
          #parser_inits
          loop {
            match crate::ast::parser::key(input) {
              Err(nom::Err::Error(_)) => {
                (input,_) = crate::ast::parser::end_group(input)?;
                let mut res = Self{#parser_posts ..Default::default()};
                <Self as crate::ast::GroupFn>::post_parse_process(&mut res, scope);
                #change_id_return
              },
              Err(e) => return Err(e),
              Ok((_input,key)) => {
                input = _input;
                #[deny(unreachable_patterns)]
                match #key_id {
                  #parser_arms
                  _ => {
                    let (new_input,undefined) = crate::ast::parser::undefine(input, key, scope)?;
                    input = new_input;
                    crate::ast::attributs_set_undefined_attri(
                      &mut #attributes_name,
                      key,
                      group_name,
                      scope,
                      undefined,
                    );
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
    Err(syn::Error::new(Span::call_site(), "Can not find NamedField".to_string()))
  }
}

#[test]
fn main() {
  use syn::parse_str;
  let input = r#"
  #[derive(liberty_macros::Group)]
  struct Timing {
    /// group attributes attributes
  #[liberty(attributes)]
    pub attributes: Attributes,
    /// group comments
  #[liberty(comments)]
    pub pub comments: GroupComments<Self>,
    #[liberty(complex)]
    values: Vec<f64>,
    #[liberty(simple(type = Option))]
    t1: Option<TimingType>,
    #[liberty(simple(type = Option))]
    t2: Option<TimingType>,
  }"#;
  let ast: &syn::DeriveInput = &parse_str(input).unwrap();
  let out = inner(ast).unwrap_or_else(|err| err.to_compile_error());
  println!("{}", out)
}
