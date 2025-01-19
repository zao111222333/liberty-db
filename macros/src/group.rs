use crate::attribute::*;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, GenericArgument, PathArguments, Type};

fn group_field_fn(
  field_name: &Ident,
  field_type: &Type,
  default: Option<&Expr>,
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
  let mut builder_init = if let Some(default) = default {
    quote! { #field_name: #default, }
  } else {
    quote! { #field_name: Default::default(), }
  };
  let wrapper_parser_arm =
    |_s_field_name: &String, parser_arm: proc_macro2::TokenStream| {
      if hashmatch {
        quote!(
          hashmatch::hash_arm!(#_s_field_name) => {
            #parser_arm
          },
        )
      } else {
        quote!(
          #_s_field_name => {
            #parser_arm
          },
        )
      }
    };
  let parser_arm: proc_macro2::TokenStream;
  let builder_field: proc_macro2::TokenStream;
  let build_arm: proc_macro2::TokenStream;
  match arrti_type {
    AttriType::Simple(SimpleType::Option) => {
      let ty = extract_generic_param(field_type, "Option")?;
      write_field = quote! {
        if let Some(simple) = &self.#field_name {
          crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
          crate::ast::SimpleAttri::fmt_liberty(simple, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,simple_res) = <#ty as crate::ast::SimpleAttri>::nom_parse(input, scope)?;
          input = new_input;
          match simple_res {
            Ok(simple) => {
              res.#field_name = Some(simple);
            },
            Err(undefined) => {
              log::error!("Line={}; Key={}; Value={:?}",scope.line_num,key,undefined);
              crate::ast::attributs_set_undefined_simple(&mut res.#attributes_name, key, undefined);
            },
          }
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Option<<#ty as crate::ast::ParsingBuilder>::Builder>,
      };
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .map(|t| crate::ast::ParsingBuilder::build(t, scope)),
      };
    }
    AttriType::Simple(SimpleType::Default) => {
      write_field = quote! {
        crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
        crate::ast::SimpleAttri::fmt_liberty(&self.#field_name, #s_field_name, f)?;
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,simple_res) = <#field_type as crate::ast::SimpleAttri>::nom_parse(input, scope)?;
          input = new_input;
          match simple_res {
            Ok(simple) => {
              res.#field_name = simple;
            },
            Err(undefined) => {
              log::error!("Line={}; Key={}; Value={:?}",scope.line_num,key,undefined);
              crate::ast::attributs_set_undefined_simple(&mut res.#attributes_name, key, undefined);
            },
          }
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: <#field_type as crate::ast::ParsingBuilder>::Builder,
      };
      build_arm = quote! {
        #field_name: builder
          .#field_name,
      };
    }
    AttriType::Complex(ComplexType::Default) => {
      write_field = quote! {
        crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
        crate::ast::ComplexAttri::fmt_liberty(&self.#field_name, #s_field_name, f)?;
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,complex_res) = <#field_type as crate::ast::ComplexAttri>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res {
            Ok(complex) => res.#field_name = complex,
            Err((e,undefined)) => {
              log::error!("Line={}; Key={}; Value={:?}; Err={}",scope.line_num,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut res.#attributes_name, key, undefined);
            },
          }
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: <#field_type as crate::ast::ParsingBuilder>::Builder,
      };
      build_arm = quote! {
        #field_name: builder
          .#field_name,
      };
    }
    AttriType::Complex(ComplexType::Option) => {
      let ty = extract_generic_param(field_type, "Option")?;
      write_field = quote! {
        if let Some(complex) = &self.#field_name {
          crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,complex_res) = <#ty as crate::ast::ComplexAttri>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res {
            Ok(complex) => res.#field_name = Some(complex),
            Err((e,undefined)) => {
              log::error!("Line={}; Key={}; Value={:?}; Err={}",scope.line_num,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut res.#attributes_name, key, undefined);
            },
          }
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Option<<#ty as crate::ast::ParsingBuilder>::Builder>,
      };
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .map(|t| crate::ast::ParsingBuilder::build(t, scope)),
      };
    }
    AttriType::Complex(ComplexType::Vec) => {
      let ty = extract_generic_param(field_type, "Vec")?;
      comment_fn = quote! {};
      write_field = quote! {
        for complex in self.#field_name.iter(){
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,complex_res) = <#ty as crate::ast::ComplexAttri>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res{
            Ok(complex) => {
              res.#field_name.push(complex);
            },
            Err((e,undefined)) => {
              log::error!("Line={}; Key={}; Value={:?}; Err={}",scope.line_num,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut res.#attributes_name, key, undefined);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Vec<<#ty as crate::ast::ParsingBuilder>::Builder>,
      };
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .into_iter()
          .map(|t| crate::ast::ParsingBuilder::build(t, scope))
          .collect(),
      };
    }
    AttriType::Complex(ComplexType::Set) => {
      let ty = extract_generic_param(field_type, "GroupSet")?;
      comment_fn = quote! {};
      write_field = quote! {
        for complex in self.#field_name.iter(){
          crate::ast::ComplexAttri::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,complex_res) = <#ty as crate::ast::ComplexAttri>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res {
            Ok(complex) => {
              res.#field_name.push(complex);
            },
            Err((e,undefined)) => {
              log::error!("Line={}; Key={}; Value={:?}; Err={}",scope.line_num,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut res.#attributes_name, key, undefined);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Vec<<#ty as crate::ast::ParsingBuilder>::Builder>,
      };
      build_arm = quote! {
        #field_name: {
          let mut map: crate::ast::GroupSet<#ty> = builder
            .#field_name
            .into_iter()
            .map(|t| crate::ast::ParsingBuilder::build(t, scope))
            .collect();
          map.sort();
          map
        },
      };
    }
    AttriType::Group(GroupType::Vec) => {
      let ty = extract_generic_param(field_type, "Vec")?;
      comment_fn = quote! {};
      write_field = quote! {
        for group in self.#field_name.iter(){
          crate::ast::fmt_comment_liberty(group.#comment_this_fn(), f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,group_res) = <#ty as crate::ast::GroupAttri>::nom_parse(input, key, scope)?;
          input = new_input;
          match group_res{
            Ok(group) => {
              res.#field_name.push(group);
            },
            Err(e) => {
              log::error!("Line={}, error={}",scope.line_num,e);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Vec<<#ty as crate::ast::ParsingBuilder>::Builder>,
      };
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .into_iter()
          .map(|t| crate::ast::ParsingBuilder::build(t, scope))
          .collect(),
      };
    }
    AttriType::Group(GroupType::Set) => {
      let ty = extract_generic_param(field_type, "GroupSet")?;
      comment_fn = quote! {};
      write_field = quote! {
        for group in self.#field_name.iter(){
          crate::ast::fmt_comment_liberty(group.#comment_this_fn(), f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,group_res) = <#ty as crate::ast::GroupAttri>::nom_parse(input, key, scope)?;
          input = new_input;
          match group_res{
            Ok(group) => {
              res.#field_name.push(group);
            },
            Err(e) => {
              log::error!("Line={}, error={}",scope.line_num,e);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Vec<<#ty as crate::ast::ParsingBuilder>::Builder>,
      };
      build_arm = quote! {
        #field_name: {
          let mut map: crate::ast::GroupSet<#ty> = builder
            .#field_name
            .into_iter()
            .map(|t| crate::ast::ParsingBuilder::build(t, scope))
            .collect();
          map.sort();
          map
        },
      };
    }
    AttriType::Group(GroupType::Option) => {
      comment_fn = quote! {};
      let ty = extract_generic_param(field_type, "Option")?;
      write_field = quote! {
        if let Some(group) = &self.#field_name {
          crate::ast::fmt_comment_liberty(group.#comment_this_fn(), f)?;
          crate::ast::GroupAttri::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        &s_field_name,
        quote! {
          let (new_input,group_res) = <#ty as crate::ast::GroupAttri>::nom_parse(input, key, scope)?;
          input = new_input;
          match group_res{
            Ok(group) => {
              if let Some(old) = res.#field_name{
                let e = crate::ast::IdError::RepeatAttri;
                log::error!("Line={}, error={}",scope.line_num,e);
              }
              res.#field_name = Some(group);
            },
            Err(e) => {
              log::error!("Line={}, error={}",scope.line_num,e);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Option<<#ty as crate::ast::ParsingBuilder>::Builder>,
      };
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .map(|t| crate::ast::ParsingBuilder::build(t, scope)),
      };
    }
    AttriType::SuperGroup(sub_groups) => {
      comment_fn = quote! {};
      write_field = quote! {
        if let Some(group) = &self.#field_name {
          group.fmt_liberty(#s_field_name, f)?;
        }
      };
      let mut _parser_arm = Vec::new();
      let mut _builder_field = Vec::new();
      let mut _build_arm = Vec::new();
      let mut _builder_init = Vec::new();
      for (sub_name, sub_type) in sub_groups {
        let ty = extract_generic_param(sub_type, "Option")?;
        _parser_arm.push(wrapper_parser_arm(
          &sub_name.to_string(),quote! {
          let (new_input,group_res) = <#ty as crate::ast::GroupAttri>::nom_parse(input, key, scope)?;
          input = new_input;
          match group_res {
            Ok(group) => {
              if let Some(old) = res.#sub_name{
                let e = crate::ast::IdError::RepeatAttri;
                log::error!("Line={}, error={}",scope.line_num,e);
              }
              res.#sub_name = Some(group);
            },
            Err(e) => {
              log::error!("Line={}, error={}",scope.line_num,e);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.line_num += n;
        }));
        _builder_field.push(quote! {
          pub(crate) #sub_name: Option<<#ty as crate::ast::ParsingBuilder>::Builder>,
        });
        _build_arm.push(quote! {builder.#sub_name,});
        _builder_init.push(quote! {#sub_name: Default::default(),});
      }
      parser_arm = quote! { #(#_parser_arm)* };
      builder_field = quote! { #(#_builder_field)* };
      build_arm = quote! {
        #field_name: crate::ast::ParsingBuilder::build((
          #(#_build_arm)*
        ), scope),
      };
      builder_init = quote! { #(#_builder_init)* };
    }
  }
  Ok((comment_fn, write_field, builder_init, parser_arm, builder_field, build_arm))
}

pub(crate) fn inner(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream> {
  let ident = &ast.ident;
  let st = match &ast.data {
    Data::Struct(s) => s,
    _ => {
      return Err(syn::Error::new(Span::call_site(), "This macro only supports struct."))
    }
  };
  let builder_ident = Ident::new(&format!("{}Builder", ident), Span::call_site());
  if let Fields::Named(named) = &st.fields {
    let fields = &named.named;
    let (
      attri_type_map,
      default_map,
      name_vec,
      attributes_name,
      comments_name,
      extra_ctx_name,
      _extra_ctx_type,
    ) = parse_fields_type(fields)?;
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
    let mut builder_fields = quote! {
      pub(crate) #attributes_name: crate::ast::Attributes,
      ____p: core::marker::PhantomData<C>,
    };
    let mut builder_inits = quote! {
      #attributes_name: Default::default(),
      ____p: core::marker::PhantomData,
    };
    let mut build_arms = quote! {
      #comments_name: Default::default(),
      #attributes_name: builder.#attributes_name,
      #extra_ctx_name: Default::default(),
    };
    for name_field in &name_vec {
      let i = name_field.ident.clone().unwrap();
      let t = &name_field.ty;
      builder_fields = quote! {
        #builder_fields
        pub(crate) #i: #t,
      };
      builder_inits = quote! {
        #builder_inits
        #i: Default::default(),
      };
      build_arms = quote! {
        #build_arms
        #i: builder.#i,
      };
    }
    let mut parser_arms = quote! {};
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
          field_name_arrti_type_old_pos
            .push((field_name, arrti_type, old_pos, &field.ty));
        }
      } else {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          "Can not find field ident!".to_string(),
        ));
      }
    }
    field_name_arrti_type_old_pos.sort_by(|(_, _, a, _), (_, _, b, _)| match (a, b) {
      (None, None) => std::cmp::Ordering::Equal,
      (None, Some(_)) => std::cmp::Ordering::Greater,
      (Some(_), None) => std::cmp::Ordering::Less,
      (Some(a), Some(b)) => a.cmp(b),
    });
    #[expect(clippy::needless_bool)]
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
    for (field_name, arrti_type, _, field_type) in field_name_arrti_type_old_pos {
      let (comment_fn, write_field, builder_init, parser_arm, builder_field, build_arm) =
        group_field_fn(
          field_name,
          field_type,
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
      builder_inits = quote! {
        #builder_inits
        #builder_init
      };
      parser_arms = quote! {
        #parser_arms
        #parser_arm
      };
      builder_fields = quote! {
        #builder_fields
        #builder_field
      };
      build_arms = quote! {
        #build_arms
        #build_arm
      };
      match arrti_type {
        AttriType::Simple(_) | AttriType::Complex(_) => {
          write_simple_complex = quote! {
            #write_simple_complex
            #write_field
          }
        }
        AttriType::Group(_) | AttriType::SuperGroup(_) => {
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
          impl<C: crate::Ctx> crate::ast::NamedGroup for #ident<C> {
            #[inline]
            fn parse_set_name(builder: &mut Self::Builder, v: Vec<&str>) -> Result<(), crate::ast::IdError> {
              <#t as crate::ast::NameAttri>::parse(v).map(|name| {builder.#i = name;})
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
      impl<C: crate::Ctx> Default for #ident<C> {
        #[inline]
        fn default() -> Self {
          Self {
            #defalut_impl
          }
        }
      }
      #named_group_impl
      #[expect(dead_code)]
      impl<C: crate::Ctx> #ident<C> {
        #comment_fns
      }
      #[expect(clippy::field_scoped_visibility_modifiers,clippy::redundant_pub_crate)]
      pub(crate) struct #builder_ident<C: crate::Ctx> {
        #builder_fields
      }
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: crate::Ctx> crate::ast::Group for #ident<C> {}
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: crate::Ctx> crate::ast::ParsingBuilder for #ident<C> {
        type Builder = #builder_ident<C>;
        fn build(mut builder: Self::Builder, scope: &mut crate::ast::BuilderScope) -> Self {
          <Self as crate::ast::GroupFn>::before_build(&mut builder, scope);
          let mut g = Self {
            #build_arms
          };
          <Self as crate::ast::GroupFn>::after_build(&mut g, scope);
          g
        }
      }
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: crate::Ctx> crate::ast::GroupAttri for #ident<C> {
        fn fmt_liberty<T: core::fmt::Write, I: crate::ast::Indentation>(&self, key: &str, f: &mut crate::ast::CodeFormatter<'_, T, I>) -> core::fmt::Result {
          use core::fmt::Write;
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
        ) -> nom::IResult<&'a str, Result<Self::Builder, crate::ast::IdError>, nom::error::Error<&'a str>> {
          let (mut input,title) = crate::ast::parser::title(i, &mut scope.line_num)?;
          let mut res = #builder_ident{#builder_inits};
          loop {
            match crate::ast::parser::key(input) {
              Err(nom::Err::Error(_)) => {
                (input,_) = crate::ast::parser::end_group(input)?;
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
                      &mut res.#attributes_name,
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
  pub(crate) struct Timing<C: Ctx> {
    /// group undefined attributes
    #[liberty(attributes)]
    attributes: Attributes,
    /// group comments
    #[liberty(comments)]
    comments: GroupComments,
    #[liberty(extra_ctx)]
    pub extra_ctx: C::Other,
    #[liberty(complex)]
    #[default = vec![0.0]]
    pub values: Vec<f64>,
  }"#;
  let ast: &syn::DeriveInput = &parse_str(input).unwrap();
  let out = inner(ast).unwrap_or_else(|err| err.to_compile_error());
  println!("{}", out)
}
fn extract_generic_param<'a>(ty: &'a Type, container: &str) -> syn::Result<&'a Type> {
  if let Type::Path(type_path) = ty {
    if let Some(last_segment) = type_path.path.segments.last() {
      if last_segment.ident == container {
        if let PathArguments::AngleBracketed(ref args) = last_segment.arguments {
          if let Some(GenericArgument::Type(ref inner_type)) = args.args.first() {
            return Ok(inner_type);
          }
        }
      }
    }
  }
  Err(syn::Error::new(
    Span::call_site(),
    format!("Can NOT extract type from {}", quote::quote!(#ty)),
  ))
}
#[test]
fn main_extended() {
  if let Ok(inner) = extract_generic_param(&syn::parse_str("Vec<TypeA>").unwrap(), "Vec")
  {
    println!("Vec inner type: {}", quote::quote!(#inner));
  }
  if let Ok(inner) =
    extract_generic_param(&syn::parse_str("Option<Vec<TypeA>>").unwrap(), "Option")
  {
    println!("Option inner type: {}", quote::quote!(#inner));
  }
}
