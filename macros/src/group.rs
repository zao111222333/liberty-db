use crate::attribute::*;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, GenericArgument, Path, PathArguments, Type};

#[expect(clippy::too_many_arguments)]
fn group_field_fn(
  field_idx: u64,
  field_name: &Ident,
  field_type: &Type,
  default: Option<&Expr>,
  before_build: Option<&Path>,
  after_build: Option<&Path>,
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
  let mut builder_init = if let Some(default) = default {
    quote! { #field_name: #default, }
  } else {
    quote! { #field_name: Default::default(), }
  };
  let ty = extract_type(field_type);
  let before_build = if let Some(f) = before_build {
    quote! { Some(#f) }
  } else {
    quote! { None }
  };
  let after_build = if let Some(f) = after_build {
    quote! { Some(#f) }
  } else {
    quote! { None }
  };
  let mut builder_field = quote! {
    pub(crate) #field_name: <#field_type as crate::ast::ParsingSet<C,#ty>>::BuilderSet,
  };
  let mut build_arm = quote! {
    #field_name: <#field_type as crate::ast::ParsingSet<C,#ty>>::build_set(builder.#field_name, scope, #before_build, #after_build),
  };
  let parser_arm: proc_macro2::TokenStream;
  let write_field: proc_macro2::TokenStream;
  match arrti_type {
    AttriType::Simple => {
      write_field = quote! {
        for simple in <#field_type as crate::ast::ParsingSet<C,#ty>>::iter_set(&self.#field_name) {
          crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
          crate::ast::SimpleAttri::<C>::fmt_liberty(simple, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        #s_field_name => {
          let (new_input,simple_res) = <#ty as crate::ast::SimpleAttri<C>>::nom_parse(input, scope)?;
          input = new_input;
          match simple_res {
            Ok(simple) => {
              <#field_type as crate::ast::ParsingSet<C,#ty>>::push_set(&mut builder.#field_name, simple, scope);
            },
            Err(undefined) => {
              crate::error!("{} Key={}; Value={:?}",scope.loc,key,undefined);
              crate::ast::attributs_set_undefined_simple(&mut builder.#attributes_name, key, undefined);
            },
          }
        }
      };
    }
    AttriType::Complex => {
      write_field = quote! {
        for complex in <#field_type as crate::ast::ParsingSet<C,#ty>>::iter_set(&self.#field_name) {
          crate::ast::fmt_comment_liberty(self.#comment_fn_name(), f)?;
          crate::ast::ComplexAttri::<C>::fmt_liberty(complex, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        #s_field_name => {
          let (new_input,complex_res) = <#ty as crate::ast::ComplexAttri<C>>::nom_parse(input, scope)?;
          input = new_input;
          match complex_res {
            Ok(complex) => {
              <#field_type as crate::ast::ParsingSet<C,#ty>>::push_set(&mut builder.#field_name, complex, scope);
            },
            Err((e,undefined)) => {
              crate::error!("{} Key={}; Value={:?}; Err={}",scope.loc,key,undefined,e);
              crate::ast::attributs_set_undefined_complex(&mut builder.#attributes_name, key, undefined);
            },
          }
        }
      };
    }
    AttriType::Group => {
      comment_fn = quote! {};
      write_field = quote! {
        for group in <#field_type as crate::ast::ParsingSet<C,#ty>>::iter_set(&self.#field_name) {
          crate::ast::fmt_comment_liberty(group.#comment_this_fn(), f)?;
          crate::ast::GroupAttri::<C>::fmt_liberty(group, #s_field_name, f)?;
        }
      };
      parser_arm = quote! {
        #s_field_name => {
          let mut group_builder = <#ty as crate::ast::ParsingBuilder<C>>::Builder::default();
          let (new_input,group_res) = <#ty as crate::ast::GroupAttri<C>>::nom_parse::<false>(&mut group_builder, input, key, scope)?;
          input = new_input;
          match group_res {
            Ok(_) => {
              <#field_type as crate::ast::ParsingSet<C,#ty>>::push_set(&mut builder.#field_name, group_builder, scope);
            },
            Err(e) => {
              crate::error!("{} error={}",scope.loc,e);
            },
          }
          let n: usize;
          (input,n) = crate::ast::parser::comment_space_newline(input)?;
          scope.loc.line_num += n;
        }
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
        let ty = extract_type(sub_type);
        let sub_name_str = sub_name.to_string();
        _parser_arm.push(quote! {
          #sub_name_str => {
            let mut group_builder = <#ty as crate::ast::ParsingBuilder<C>>::Builder::default();
            let (new_input,group_res) = <#ty as crate::ast::GroupAttri<C>>::nom_parse::<false>(&mut group_builder, input, key, scope)?;
            input = new_input;
            match group_res {
              Ok(_) => {
                <#sub_type as crate::ast::ParsingSet<C,#ty>>::push_set(&mut builder.#sub_name, group_builder, scope);
              },
              Err(e) => {
                crate::error!("{} error={}",scope.loc,e);
              },
            }
            let n: usize;
            (input,n) = crate::ast::parser::comment_space_newline(input)?;
            scope.loc.line_num += n;
          }
        });
        _builder_field.push(quote! {
          pub(crate) #sub_name: <#sub_type as crate::ast::ParsingSet<C,#ty>>::BuilderSet,
        });
        _build_arm.push(quote! {builder.#sub_name,});
        _builder_init.push(quote! {#sub_name: Default::default(),});
      }
      parser_arm = quote! { #(#_parser_arm)* };
      builder_field = quote! { #(#_builder_field)* };
      build_arm = quote! {
        #field_name: {
          let t = ( #(#_build_arm)* );
          <#field_type as crate::ast::ParsingBuilder<C>>::build_full(t, scope, #before_build, #after_build)
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
  let builder_ident = Ident::new(&format!("{ident}Builder"), Span::call_site());
  if let Fields::Named(named) = &st.fields {
    let fields = &named.named;
    let FieldsType {
      attri_type_map,
      default_map,
      dynamic_name_map,
      before_build_map,
      after_build_map,
      name_vec,
      attributes_name,
      comments_name,
      extra_ctx_name,
    } = parse_fields_type(fields)?;
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
        AttriType::Simple | AttriType::Complex => {
          write_simple_complex = quote! {
            #write_simple_complex
            #write_field
          }
        }
        AttriType::Group | AttriType::SuperGroup(_) => {
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
          f.write_new_line_indentation()?;
          write!(f,"{key} () {{")?;
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
          f.write_new_line_indentation()?;
          write!(f,"{key} (")?;
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
            impl<C: 'static + crate::Ctx> crate::ast::NamedGroup<C> for #ident<C> {
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
            impl<C: 'static + crate::Ctx> crate::ast::NamedGroup<C> for #ident<C> {
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
      impl<C: 'static + crate::Ctx> Default for #ident<C> {
        #[inline]
        fn default() -> Self {
          Self {
            #defalut_impl
          }
        }
      }
      #named_group_impl
      #[expect(dead_code)]
      impl<C: 'static + crate::Ctx> #ident<C> {
        #comment_fns
      }
      #[expect(clippy::field_scoped_visibility_modifiers,clippy::redundant_pub_crate)]
      #[derive(Clone, Debug)]
      pub(crate) struct #builder_ident<C: 'static + crate::Ctx> {
        #builder_fields
      }
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: 'static + crate::Ctx> crate::ast::Group<C> for #ident<C> {}
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: 'static + crate::Ctx> crate::ast::ParsingBuilder<C> for #ident<C> {
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
      impl<C: 'static + crate::Ctx> Default for #builder_ident<C> {
        fn default() -> Self {
          Self{#builder_inits}
        }
      }
      #[doc(hidden)]
      #[allow(non_upper_case_globals, unused_attributes, unused_qualifications, clippy::too_many_lines)]
      impl<C: 'static + crate::Ctx> crate::ast::GroupAttri<C> for #ident<C> {
        fn fmt_liberty<T: core::fmt::Write, I: crate::ast::Indentation>(&self, key: &str, f: &mut crate::ast::CodeFormatter<'_, T, I>) -> core::fmt::Result {
          use core::fmt::Write;
          #write_title
          f.indent();
          #write_simple_complex
          if !self.#attributes_name.is_empty(){
            crate::ast::attributs_fmt_liberty(&self.#attributes_name,f)?;
          }
          #write_group
          f.dedent();
          f.write_new_line_indentation()?;
          write!(f, "}}")
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
  pub(crate) struct Timing<C: 'static+Ctx> {
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
fn extract_type(ty: &Type) -> &Type {
  if let Type::Path(type_path) = ty
    && let Some(last_segment) = type_path.path.segments.last()
    && (last_segment.ident == "Option"
      || last_segment.ident == "LibertyVec"
      || last_segment.ident == "LibertySet")
    && let PathArguments::AngleBracketed(ref args) = last_segment.arguments
    && let Some(GenericArgument::Type(inner_type)) = args.args.first()
  {
    return inner_type;
  }
  ty
}
#[test]
fn main_extended() {
  let ty = syn::parse_str("WordSet").unwrap();
  let ty = extract_type(&ty);
  println!("Vec inner type: {}", quote::quote!(#ty));
  let ty = syn::parse_str("Vec<TypeA>").unwrap();
  let ty = extract_type(&ty);
  println!("Vec inner type: {}", quote::quote!(#ty));
  let ty = syn::parse_str("Option<Vec<TypeA>>").unwrap();
  let ty = extract_type(&ty);
  println!("Vec inner type: {}", quote::quote!(#ty));
}

#[test]
fn test_group() {
  use syn::parse_str;
  let input = r#"
  pub struct DynamicCurrent<C: 'static+Ctx> {
    #[liberty(name)]
    #[id]
    pub name: Option<String>,
    /// group comments
    #[liberty(comments)]
    comments: GroupComments,
    #[liberty(extra_ctx)]
    pub extra_ctx: C::Other,
    /// group undefined attributes
    #[liberty(attributes)]
    pub attributes: crate::ast::Attributes,
    #[id]
    #[liberty(simple)]
    pub when: Option<LogicBooleanExpression>,
    #[id]
    #[liberty(simple)]
    pub related_inputs: WordSet,
    #[id]
    #[liberty(simple)]
    pub related_outputs: WordSet,
    #[liberty(complex)]
    pub typical_capacitances: Option<Vec<f64>>,
    /// Use the switching_group group to specify a current waveform vector when the power
    /// and ground current is dependent on pin switching conditions.
    /// <a name ="reference_link" href="
    /// https://zao111222333.github.io/liberty-db/2020.09/reference_manual.html?field=null&bgn=150.18&end=150.19
    /// ">Reference</a>
    #[liberty(group)]
    pub switching_group: LibertySet<SwitchingGroup<C>>,
  }
    "#;
  let item: DeriveInput = parse_str(input).unwrap();
  let out = inner(&item).unwrap_or_else(|err| err.to_compile_error());
  println!("{}", out)
}
