use proc_macro2::Span;
use quote::quote;
use syn::{Attribute, DeriveInput, NestedMeta};
use syn::{Data, Fields};

#[derive(Debug)]
enum IdxLen {
  Num(usize),
  Vector,
  ParserError,
  OnlyImplHashEq,
}

fn parse_id_len(field_attrs: &Vec<Attribute>) -> IdxLen {
  for attri in field_attrs.into_iter() {
    if let Some(seg_title) = attri.path.segments.first() {
      if "id_len" == &seg_title.ident.to_string() {
        if let Ok(NestedMeta::Lit(syn::Lit::Int(n))) = attri.parse_args::<NestedMeta>() {
          match n.base10_parse::<isize>() {
            Ok(n) => match usize::try_from(n) {
              Ok(n) => return IdxLen::Num(n),
              Err(_) => return IdxLen::Vector,
            },
            Err(_) => return IdxLen::ParserError,
          }
        } else {
          return IdxLen::ParserError;
        }
      }
    }
  }
  return IdxLen::OnlyImplHashEq;
}

fn find_id_len(
  fields: &syn::punctuated::Punctuated<syn::Field, syn::token::Comma>,
) -> Option<IdxLen> {
  for field in fields.into_iter() {
    if let Some(id) = &field.ident {
      if "_id" == &id.to_string() {
        return Some(parse_id_len(&field.attrs));
      }
    }
  }
  return None;
}

fn group_field_fn(
  field_name: &proc_macro2::Ident,
  field_type: Option<(Option<FieldType>, String)>,
  arrti_type: AttriType,
) -> syn::Result<(proc_macro2::TokenStream, proc_macro2::TokenStream)> {
  let s_field_name = field_name.to_string();
  let write_field: _;
  let parser_arm: _;
  match arrti_type {
    AttriType::Unkown(s) => {
      return Err(syn::Error::new(
        Span::call_site(),
        format!("Unsupported type: {s}, support: [simple,complex,group]"),
      ))
    }
    AttriType::Simple => {
      match field_type {
        Some((Some(FieldType::Option), _)) => {
          write_field = quote! {
            if let Some(simple) = &self.#field_name {
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
                // res.add_undefine_attri(key,attri);
                res.undefined_list().push((key.to_owned(), undefined));
              },
            }
          };
        }
        _ => {
          write_field = quote! {
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
                // res.add_undefine_attri(key,attri);
                res.undefined_list().push((key.to_owned(), undefined));
              },
            }
          };
        }
      }
    }
    AttriType::Complex => {
      write_field = quote! {
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
    AttriType::Group => match field_type {
      Some((Some(FieldType::Vector), _)) => {
        write_field = quote! {
          for group in self.#field_name.iter(){
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
      Some((Some(FieldType::HashSet), _)) => {
        write_field = quote! {
          for group in self.#field_name.iter(){
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
      Some((Some(FieldType::Option), _)) => {
        write_field = quote! {
          if let Some(group) = &self.#field_name {
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
      Some((None, ty)) => {
        return Err(syn::Error::new(
          Span::call_site(),
          format!("Unsupported field={}, type={}", s_field_name, ty),
        ))
      }
      None => {
        return Err(syn::Error::new(
          Span::call_site(),
          format!("Unsupported field={}, type=?", s_field_name),
        ))
      }
    },
  };
  Ok((
    write_field,
    quote!(
      #s_field_name => {
        #parser_arm
      },
    ),
  ))
}
fn name_id(
  name: &proc_macro2::Ident,
  id_len: &IdxLen,
) -> syn::Result<proc_macro2::TokenStream> {
  // let id_name = quote::format_ident!("{}Id", name);
  let impl_trait = quote! {
    impl Eq for #name {}
    impl PartialEq for #name {
      fn eq(&self, other: &Self) -> bool {
        <#name as crate::ast::HashedGroup>::id(&self) == <#name as crate::ast::HashedGroup>::id(other)
      }
    }
    impl std::hash::Hash for #name {
      fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        <#name as crate::ast::HashedGroup>::id(&self).hash(state);
      }
    }
    impl std::borrow::Borrow<<#name as crate::ast::HashedGroup>::Id> for #name{
      fn borrow(&self) -> &<Self as crate::ast::HashedGroup>::Id {
        <#name as crate::ast::HashedGroup>::id(&self)
      }
  }
  };
  let fn_id = quote! {
    #[inline]
    fn id(&self) -> &Self::Id {
      &self._id
    }
  };
  let toks = match id_len {
    IdxLen::OnlyImplHashEq => quote! {
      #impl_trait
    },
    IdxLen::ParserError => {
      return Err(syn::Error::new(Span::call_site(), "`id_len` parse error"))
    }
    IdxLen::Num(0) => {
      return Err(syn::Error::new(Span::call_site(), "`id_len` should larger than `1`"))
    }
    IdxLen::Num(1) => quote! {
      #impl_trait
      impl crate::ast::HashedGroup for #name {
        type Id=String;
        #[inline]
        fn title(&self) -> Vec<String>{
          vec![self._id.clone()]
        }
        #fn_id
        #[inline]
        fn gen_id(&self, mut title: Vec<String>) -> Result<Self::Id,crate::ast::IdError>{
          let l=title.len();
          if l!=1{
              return Err(crate::ast::IdError::LengthDismatch(1,l,title));
          }
          if let Some(name) = title.pop(){
            Ok(name)
          }else{
            return Err(crate::ast::IdError::Other("Unkown pop error".into()));
          }
        }
      }
    },
    IdxLen::Num(c) => quote! {
      #impl_trait
      impl crate::ast::HashedGroup for #name {
        type Id=[String;#c];
        #[inline]
        fn title(&self) -> Vec<String>{
          self._id.clone().to_vec()
        }
        #fn_id
        #[inline]
        fn gen_id(&self, title: Vec<String>) -> Result<Self::Id,crate::ast::IdError>{
          let l=title.len();
          if l!=#c{
              return Err(crate::ast::IdError::LengthDismatch(#c,l,title));
          }
          match TryInto::<[String; #c ]>::try_into(title){
            Ok(name) => Ok(name),
            Err(e) => Err(crate::ast::IdError::Other(format!("try_into error: {:?}",e))),
          }
        }
      }
    },
    IdxLen::Vector => quote! {
      #impl_trait
      impl crate::ast::HashedGroup for #name {
        type Id=Vec<String>;
        #[inline]
        fn title(&self) -> Vec<String>{
          self._id.clone()
        }
        #fn_id
        #[inline]
        fn gen_id(&self, title: Vec<String>) -> Result<Self::Id,crate::ast::IdError>{
          Ok(title)
        }
      }
    },
  };
  Ok(toks)
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
    let (hashed, impl_hashed_group) = match find_id_len(fields) {
      Some(id_len) => (true, name_id(name, &id_len)?),
      None => (false, quote! {}),
    };
    let mut parser_arms = quote! {};
    let mut write_fields = quote! {};
    for field in fields.into_iter() {
      if let (Some(field_name), field_attrs) = (&field.ident, &field.attrs) {
        if let Some(arrti_type) = parse_field_attrs(field_attrs) {
          let (write_field, parser_arm) =
            group_field_fn(field_name, extract_type(&field.ty), arrti_type)?;
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
    }
    let change_id_return = if hashed {
      quote! {
        match crate::ast::HashedGroup::gen_id(&res,title){
          Ok(id) => {
            res._id = id;
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
    let write_title = if hashed {
      quote! {
        write!(f,"\n{} ({}) {{", key, crate::ast::HashedGroup::title(self).iter().map(
          |s|
          if crate::ast::is_word(s){
            s.clone()
          }else{
            "\"".to_owned()+s+"\""
          }
        ).join(","))?;
      }
    } else {
      quote! {
        write!(f,"\n{} () {{",key)?;
      }
    };
    let impl_group = quote! {
      impl crate::ast::GroupAttri for #name {
        #[inline]
        fn undefined_list(&mut self)-> &mut crate::ast::AttributeList{
          &mut self._undefined
        }
        fn fmt_liberty<T: std::fmt::Write>(&self, key: &str, f: &mut crate::ast::CodeFormatter<'_, T>) -> std::fmt::Result {
          use std::fmt::Write;
          use itertools::Itertools;
          #write_title
          f.indent(1);
          #write_fields
          if !self._undefined.is_empty(){
            write!(f, "\n/* Undefined attributes from here */")?;
            crate::ast::liberty_attr_list(&self._undefined,f)?;
          }
          f.dedent(1);
          write!(f, "\n}}")
        }
        fn nom_parse<'a>(
          i: &'a str, line_num: &mut usize
        ) -> nom::IResult<&'a str, Result<Self,crate::ast::IdError>, nom::error::Error<&'a str>> {
          let (mut input,title) = crate::ast::parser::title(i,line_num)?;
          let mut res = Self::default();
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
                    // res.add_undefine_attri(key, undefine);
                    res.undefined_list().push((key.to_owned(), undefined));
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
      #impl_hashed_group
      #impl_group
    })
  } else {
    Err(syn::Error::new(Span::call_site(), format!("Can not find NamedField")))
  }
}

#[derive(Debug)]
enum AttriType {
  Simple,
  Complex,
  Group,
  // GroupHashed,
  Unkown(String),
}

fn parse_field_attrs(field_attrs: &Vec<Attribute>) -> Option<AttriType> {
  for attri in field_attrs.into_iter() {
    if let Some(seg_title) = attri.path.segments.first() {
      if "arrti_type" == &seg_title.ident.to_string() {
        if let Ok(NestedMeta::Meta(meta)) = attri.parse_args::<NestedMeta>() {
          if let Some(seg_type) = meta.path().segments.first() {
            let type_str = seg_type.ident.to_string();
            match type_str.as_str() {
              "simple" => return Some(AttriType::Simple),
              "complex" => return Some(AttriType::Complex),
              "group" => return Some(AttriType::Group),
              _ => return Some(AttriType::Unkown(type_str)),
            }
          }
        }
      }
    }
  }
  return None;
}

enum FieldType {
  HashSet,
  Vector,
  Option,
}

/// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
fn extract_type(ty: &syn::Type) -> Option<(Option<FieldType>, String)> {
  match *ty {
    syn::Type::Path(ref typepath) if typepath.qself.is_none() => {
      let idents_of_path =
        typepath
          .path
          .segments
          .iter()
          .into_iter()
          .fold(String::new(), |mut acc, v| {
            acc.push_str(&v.ident.to_string());
            acc.push('|');
            acc
          });
      if let Some(_) =
        vec!["Option|", "std|option|Option|", "core|option|Option|", "option|Option|"]
          .into_iter()
          .find(|s| &idents_of_path == s)
      {
        return Some((Some(FieldType::Option), idents_of_path));
      }
      if let Some(_) =
        vec!["HashSet|", "collections|HashSet|", "std|collections|HashSet|"]
          .into_iter()
          .find(|s| &idents_of_path == s)
      {
        return Some((Some(FieldType::HashSet), idents_of_path));
      }

      if let Some(_) = vec!["Vec|", "alloc|vec|Vec|"]
        .into_iter()
        .find(|s| &idents_of_path == s)
      {
        return Some((Some(FieldType::Vector), idents_of_path));
      }
      return Some((None, idents_of_path));
    }
    _ => None,
  }
}
