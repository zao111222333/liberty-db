use crate::attribute::*;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, GenericArgument, PathArguments, Type};

#[expect(clippy::too_many_arguments)]
fn group_field_fn(
  field_idx: u64,
  field_name: &Ident,
  field_type: &Type,
  default: Option<&Expr>,
  before_build: Option<&MyPath>,
  after_build: Option<&MyPath>,
  arrti_type: &AttriType,
  attributes_name: &Ident,
  comments_name: &Ident,
) -> syn::Result<(
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
  proc_macro2::TokenStream,
)> {
  let s_field_name = field_name.to_string();
  let s_field_name = s_field_name.strip_prefix("r#").unwrap_or(&s_field_name);
  let comment_fn_name =
    Ident::new(&format!("comments_{s_field_name}"), Span::call_site());
  let comment_this_fn = Ident::new("comments_this", Span::call_site());
  let comment_fn_entry =
    Ident::new(&format!("comments_{s_field_name}_entry"), Span::call_site());
  let mut comment_fn = quote! {
    #[inline]
    pub fn #comment_fn_name(&self)-> Option<&String> {
      self.#comments_name.0.get(&#field_idx)
    }
    #[inline]
    pub fn #comment_fn_entry<'a>(&'a mut self)-> std::collections::hash_map::Entry<'a, u64, String> {
      self.#comments_name.0.entry(#field_idx)
    }
  };
  let write_field: proc_macro2::TokenStream;
  let mut builder_init = if let Some(default) = default {
    quote! { #field_name: #default, }
  } else {
    quote! { #field_name: Default::default(), }
  };
  let wrapper_parser_arm = |_s_field_name: &str, parser_arm: proc_macro2::TokenStream| {
    quote!(
      #_s_field_name => {
        #parser_arm
      },
    )
  };
  let build_fn = |ty: &Type| match (before_build, after_build) {
    (None, None) => quote! { crate::ast::ParsingBuilder::<C>::build(t, scope) },
    (None, Some(after)) => {
      let __t = if after.is_macro { quote!(_t) } else { quote!(&mut _t) };
      quote! { {
        let mut _t: #ty = crate::ast::ParsingBuilder::<C>::build(t, scope);
        #after(#__t, scope);
        _t
      } }
    }
    (Some(before), None) => quote! { {
      let mut t = t;
      #before(&mut t, scope);
      crate::ast::ParsingBuilder::<C>::build(t, scope)
      _t
    } },
    (Some(before), Some(after)) => {
      let __t = if after.is_macro { quote!(_t) } else { quote!(&mut _t) };
      quote! { {
        let mut t = t;
        #before(&mut t, scope);
        let mut _t: #ty = crate::ast::ParsingBuilder::<C>::build(t, scope);
        #after(#__t, scope);
        _t
      } }
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
          crate::ast::SimpleAttri::<C>::fmt_liberty(simple, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let (new_input,simple_res) = <#ty as crate::ast::SimpleAttri<C>>::nom_parse(input, scope)?;
          input = new_input;
          match simple_res {
            Ok(simple) => {
              builder.#field_name = Some(simple);
            },
            Err(undefined) => {
              crate::error!("{} Key={}; Value={:?}",scope.loc,key,undefined);
              crate::ast::attributs_set_undefined_simple(&mut builder.#attributes_name, key, undefined);
            },
          }
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Option<<#ty as crate::ast::ParsingBuilder<C>>::Builder>,
      };
      let build = build_fn(ty);
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .map(|t| #build),
      };
    }
    AttriType::Simple(SimpleType::Default) => {
      write_field = quote! {
        crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
        crate::ast::SimpleAttri::<C>::fmt_liberty(&self.#field_name, #s_field_name, f)?;
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let (new_input,simple_res) = <#field_type as crate::ast::SimpleAttri<C>>::nom_parse(input, scope)?;
          input = new_input;
          match simple_res {
            Ok(simple) => {
              builder.#field_name = simple;
            },
            Err(undefined) => {
              crate::error!("{} Key={}; Value={:?}",scope.loc,key,undefined);
              crate::ast::attributs_set_undefined_simple(&mut builder.#attributes_name, key, undefined);
            },
          }
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: <#field_type as crate::ast::ParsingBuilder<C>>::Builder,
      };
      let build = build_fn(field_type);
      build_arm = quote! {
        #field_name: {
          let t = builder.#field_name;
          #build
        },
      };
    }
    AttriType::Complex(ComplexType::Default) => {
      write_field = quote! {
        crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
        crate::ast::ComplexAttri::<C>::fmt_liberty(&self.#field_name, #s_field_name, f)?;
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let (new_input,complex_res) = <#field_type as crate::ast::ComplexAttri<C>>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res {
            Ok(complex) => builder.#field_name = complex,
            Err((e,undefined)) => {
              crate::error!("{} Key={}; Value={:?}; Err={}",scope.loc,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut builder.#attributes_name, key, undefined);
            },
          }
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: <#field_type as crate::ast::ParsingBuilder<C>>::Builder,
      };
      let build = build_fn(field_type);
      build_arm = quote! {
        #field_name: {
          let t = builder.#field_name;
          #build
        },
      };
    }
    AttriType::Complex(ComplexType::Option) => {
      let ty = extract_generic_param(field_type, "Option")?;
      write_field = quote! {
        if let Some(complex) = &self.#field_name {
          crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
          crate::ast::ComplexAttri::<C>::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let (new_input,complex_res) = <#ty as crate::ast::ComplexAttri<C>>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res {
            Ok(complex) => builder.#field_name = Some(complex),
            Err((e,undefined)) => {
              crate::error!("{} Key={}; Value={:?}; Err={}",scope.loc,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut builder.#attributes_name, key, undefined);
            },
          }
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Option<<#ty as crate::ast::ParsingBuilder<C>>::Builder>,
      };
      let build = build_fn(ty);
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .map(|t| #build),
      };
    }
    AttriType::Complex(ComplexType::Vec) => {
      let ty = extract_generic_param(field_type, "Vec")?;
      comment_fn = quote! {};
      write_field = quote! {
        for complex in self.#field_name.iter(){
          crate::ast::ComplexAttri::<C>::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let (new_input,complex_res) = <#ty as crate::ast::ComplexAttri<C>>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res{
            Ok(complex) => {
              builder.#field_name.push(complex);
            },
            Err((e,undefined)) => {
              crate::error!("{} Key={}; Value={:?}; Err={}",scope.loc,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut builder.#attributes_name, key, undefined);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.loc.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Vec<<#ty as crate::ast::ParsingBuilder<C>>::Builder>,
      };
      let build = build_fn(ty);
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .into_iter()
          .map(|t| #build)
          .collect(),
      };
    }
    AttriType::Complex(ComplexType::Set) => {
      let ty = extract_generic_param(field_type, "GroupSet")?;
      comment_fn = quote! {};
      write_field = quote! {
        for complex in self.#field_name.iter(){
          crate::ast::ComplexAttri::<C>::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let (new_input,complex_res) = <#ty as crate::ast::ComplexAttri<C>>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res {
            Ok(complex) => {
              builder.#field_name.push(complex);
            },
            Err((e,undefined)) => {
              crate::error!("{} Key={}; Value={:?}; Err={}",scope.loc,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut builder.#attributes_name, key, undefined);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.loc.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Vec<<#ty as crate::ast::ParsingBuilder<C>>::Builder>,
      };
      let build = build_fn(ty);
      build_arm = quote! {
        #field_name: {
          let mut map: crate::ast::GroupSet<#ty> = builder
            .#field_name
            .into_iter()
            .map(|t| #build)
            .collect();
          map.sort_unstable();
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
          crate::ast::GroupAttri::<C>::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let mut group_builder = <#ty as crate::ast::ParsingBuilder<C>>::Builder::default();
          let (new_input,group_res) = <#ty as crate::ast::GroupAttri<C>>::nom_parse::<false>(&mut group_builder, input, key, scope)?;
          input = new_input;
          match group_res{
            Ok(_) => {
              builder.#field_name.push(group_builder);
            },
            Err(e) => {
              crate::error!("{} error={}",scope.loc,e);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.loc.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Vec<<#ty as crate::ast::ParsingBuilder<C>>::Builder>,
      };
      let build = build_fn(ty);
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .into_iter()
          .map(|t| #build)
          .collect(),
      };
    }
    AttriType::Group(GroupType::Set) => {
      let ty = extract_generic_param(field_type, "GroupSet")?;
      comment_fn = quote! {};
      write_field = quote! {
        for group in self.#field_name.iter(){
          crate::ast::fmt_comment_liberty(group.#comment_this_fn(), f)?;
          crate::ast::GroupAttri::<C>::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let mut group_builder = <#ty as crate::ast::ParsingBuilder<C>>::Builder::default();
          let (new_input,group_res) = <#ty as crate::ast::GroupAttri<C>>::nom_parse::<false>(&mut group_builder, input, key, scope)?;
          input = new_input;
          match group_res{
            Ok(_) => {
              builder.#field_name.push(group_builder);
            },
            Err(e) => {
              crate::error!("{} error={}",scope.loc,e);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.loc.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Vec<<#ty as crate::ast::ParsingBuilder<C>>::Builder>,
      };
      let build = build_fn(ty);
      build_arm = quote! {
        #field_name: {
          let mut map: crate::ast::GroupSet<#ty> = builder
            .#field_name
            .into_iter()
            .map(|t| #build)
            .collect();
          map.sort_unstable();
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
          crate::ast::GroupAttri::<C>::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = wrapper_parser_arm(
        s_field_name,
        quote! {
          let mut group_builder = <#ty as crate::ast::ParsingBuilder<C>>::Builder::default();
          let (new_input,group_res) = <#ty as crate::ast::GroupAttri<C>>::nom_parse::<false>(&mut group_builder, input, key, scope)?;
          input = new_input;
          match group_res{
            Ok(_) => {
              if let Some(old) = &builder.#field_name{
                let e = crate::ast::IdError::RepeatAttri;
                crate::error!("{} error={}",scope.loc,e);
              }
              builder.#field_name = Some(group_builder);
            },
            Err(e) => {
              crate::error!("{} error={}",scope.loc,e);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.loc.line_num += n;
        },
      );
      builder_field = quote! {
        pub(crate) #field_name: Option<<#ty as crate::ast::ParsingBuilder<C>>::Builder>,
      };
      let build = build_fn(ty);
      build_arm = quote! {
        #field_name: builder
          .#field_name
          .map(|t| #build),
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
            let mut group_builder = <#ty as crate::ast::ParsingBuilder<C>>::Builder::default();
            let (new_input,group_res) = <#ty as crate::ast::GroupAttri<C>>::nom_parse::<false>(&mut group_builder, input, key, scope)?;
            input = new_input;
            match group_res {
              Ok(_) => {
                if let Some(old) = &builder.#sub_name{
                  let e = crate::ast::IdError::RepeatAttri;
                  crate::error!("{} error={}",scope.loc,e);
                }
                builder.#sub_name = Some(group_builder);
              },
              Err(e) => {
                crate::error!("{} error={}",scope.loc,e);
              },
            }
            let n: usize;
            (input,n) = crate::ast::parser::comment_space_newline(input)?;
            scope.loc.line_num += n;
          }));
        _builder_field.push(quote! {
          pub(crate) #sub_name: Option<<#ty as crate::ast::ParsingBuilder<C>>::Builder>,
        });
        _build_arm.push(quote! {builder.#sub_name,});
        _builder_init.push(quote! {#sub_name: Default::default(),});
      }
      parser_arm = quote! { #(#_parser_arm)* };
      builder_field = quote! { #(#_builder_field)* };
      let build = build_fn(field_type);
      build_arm = quote! {
        #field_name: {
          let t = ( #(#_build_arm)* );
          #build
        },
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
      return Err(syn::Error::new(Span::call_site(), "This macro only supports struct."));
    }
  };
  let builder_ident = Ident::new(&format!("{}Builder", ident), Span::call_site());
  if let Fields::Named(named) = &st.fields {
    let fields = &named.named;
    let (
      attri_type_map,
      default_map,
      before_build_map,
      after_build_map,
      name_vec,
      attributes_name,
      comments_name,
      extra_ctx_name,
      _extra_ctx_type,
    ) = parse_fields_type(fields)?;
    let mut comment_fns = quote! {
      #[inline]
      pub fn comments_this(&self)-> Option<&String> {
        self.#comments_name.0.get(&0)
      }
      #[inline]
      pub fn comments_this_entry<'a>(&'a mut self)-> std::collections::hash_map::Entry<'a, u64, String> {
        self.#comments_name.0.entry(0)
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
    for (flatten, name_field) in &name_vec {
      let i = name_field.ident.clone().unwrap();
      let t = &name_field.ty;
      builder_fields = if *flatten {
        quote! {
          #builder_fields
          pub(crate) #i: Vec<#t>,
        }
      } else {
        quote! {
          #builder_fields
          pub(crate) #i: #t,
        }
      };
      builder_inits = quote! {
        #builder_inits
        #i: Default::default(),
      };
      build_arms = if *flatten {
        quote! {
          #build_arms
          #i: crate::ast::FlattenNameAttri::pretend_group(builder.#i),
        }
      } else {
        quote! {
          #build_arms
          #i: builder.#i,
        }
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
    let mut field_name_arrti_type = Vec::new();
    for field in fields.into_iter() {
      if let Some(field_name) = &field.ident {
        if let Some(arrti_type) = attri_type_map.get(field_name) {
          field_name_arrti_type.push((field_name, arrti_type, &field.ty));
        }
      } else {
        return Err(syn::Error::new(
          proc_macro2::Span::call_site(),
          "Can not find field ident!".to_string(),
        ));
      }
    }
    for (idx, (field_name, arrti_type, field_type)) in
      field_name_arrti_type.into_iter().enumerate()
    {
      let (comment_fn, write_field, builder_init, parser_arm, builder_field, build_arm) =
        group_field_fn(
          (idx + 1) as u64,
          field_name,
          field_type,
          default_map.get(field_name),
          before_build_map.get(field_name),
          after_build_map.get(field_name),
          arrti_type,
          attributes_name,
          comments_name,
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
        quote! {return Ok((input, Ok(())));},
        quote! {
          write!(f,"\n{indent}{key} () {{")?;
        },
      )
    } else {
      (
        quote! {
          return Ok((input,
            match <Self as crate::ast::NamedGroup<C>>::parse_set_name(builder, title){
              Ok(_) => {
                Ok(())
              },
              Err(e) => {
                Err(e)
              },
            }
          ));
        },
        quote! {
          write!(f,"\n{indent}{key} (")?;
          crate::ast::NamedGroup::<C>::fmt_name(self, f)?;
          write!(f,") {{")?;
        },
      )
    };
    let named_group_impl = match name_vec.len() {
      1 => {
        let flatten = name_vec[0].0;
        let t = &name_vec[0].1.ty;
        let i = name_vec[0].1.ident.as_ref().expect("name has no ident!");
        if flatten {
          quote! {
            #[doc(hidden)]
            impl<C: crate::Ctx> crate::ast::NamedGroup<C> for #ident<C> {
              #[inline]
              fn parse_set_name(builder: &mut Self::Builder, v: Vec<&str>) -> Result<(), crate::ast::IdError> {
                <#t as crate::ast::FlattenNameAttri>::parse(v).map(|name| {builder.#i = name;})
              }
              #[inline]
              fn fmt_name<T: core::fmt::Write, I: crate::ast::Indentation>(
                &self,
                f: &mut crate::ast::CodeFormatter<'_, T, I>,
              ) -> core::fmt::Result
              {
                <#t as crate::ast::FlattenNameAttri>::fmt_self(&self.#i, f)
              }
            }
          }
        } else {
          quote! {
            #[doc(hidden)]
            impl<C: crate::Ctx> crate::ast::NamedGroup<C> for #ident<C> {
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
      }
      _ => quote!(),
    };

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
      #[derive(Clone, Debug)]
      pub(crate) struct #builder_ident<C: crate::Ctx> {
        #builder_fields
      }
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: crate::Ctx> crate::ast::Group<C> for #ident<C> {}
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: crate::Ctx> crate::ast::ParsingBuilder<C> for #ident<C> {
        type Builder = #builder_ident<C>;
        fn build(mut builder: Self::Builder, scope: &mut crate::ast::BuilderScope<C>) -> Self {
          <Self as crate::ast::GroupFn<C>>::before_build(&mut builder, scope);
          let mut g = Self {
            #build_arms
          };
          <Self as crate::ast::GroupFn<C>>::after_build(&mut g, scope);
          g
        }
      }
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: crate::Ctx> Default for #builder_ident<C> {
        fn default() -> Self {
          Self{#builder_inits}
        }
      }
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: crate::Ctx> crate::ast::GroupAttri<C> for #ident<C> {
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
        #[expect(unused_variables,clippy::collection_is_never_read)]
        fn nom_parse<'a, const IS_INCLUDED: bool>(
          builder: &mut Self::Builder,
          mut input: &'a str,
          group_name: &str,
          scope: &mut crate::ast::ParseScope<'_>,
        ) -> nom::IResult<&'a str, Result<(), crate::ast::IdError>, nom::error::Error<&'a str>> {
          let title;
          if IS_INCLUDED {
            title = Default::default();
          } else {
            (input, title) = crate::ast::parser::title(input, &mut scope.loc.line_num)?;
          }
          loop {
            match crate::ast::parser::key(input) {
              Err(nom::Err::Error(_)) => {
                if IS_INCLUDED {
                  return Ok((input, Ok(())))
                } else {
                  (input,_) = crate::ast::parser::end_group(input)?;
                  #change_id_return
                }
              },
              Err(e) => return Err(e),
              Ok((_input,key)) => {
                input = _input;
                #[deny(unreachable_patterns)]
                match key {
                  #parser_arms
                  "include_file" => {
                    (input, _) = Self::include_file(builder, input, group_name, scope)?;
                  },
                  _ => {
                    if let Ok((new_input,undefined)) = crate::ast::parser::undefine(input, key, scope) {
                      input = new_input;
                      crate::ast::attributs_set_undefined_attri(
                        &mut builder.#attributes_name,
                        key,
                        group_name,
                        scope,
                        undefined,
                      );
                    } else {
                      let (new_input,_) = crate::ast::parser::variable(input, key, scope)?;
                      input = new_input;
                    }
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
          if let Some(GenericArgument::Type(inner_type)) = args.args.first() {
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
