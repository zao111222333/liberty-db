use proc_macro2::Span;
use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, Attribute, NestedMeta};
use syn::{Data, Fields};

pub(crate) fn inner(ast: &DeriveInput, hashed: bool) -> syn::Result<proc_macro2::TokenStream>{
  let name = &ast.ident;
  let st = match &ast.data {
    Data::Struct(s) => s,
    _ => return Err(syn::Error::new(Span::call_site(), "This macro only supports struct.")),
  };
  let mut parser_arms = quote!{};
  let mut to_wrappers = quote!{};
  if let Fields::Named(named) =  &st.fields{
    let fields = &named.named;
    for field in fields.into_iter(){
      if let (Some(field_name),field_attrs) = (&field.ident,&field.attrs){
        if let Some(arrti_type) = parse_field_attrs(field_attrs){
          let s_field_name = field_name.to_string();
          let to_wrapper: _;
          let parser_arm: _;
          match arrti_type{
            AttriType::Unkown(s) => return Err(syn::Error::new(Span::call_site(), 
              format!("Unsupported type: {s}, support: [simple,complex,group,group_hashed]")
            )),
            AttriType::Simple => {
              match extract_type_from_option(&field.ty){
                // for Option<xxx>
                Some(_) => {
                  to_wrapper = quote!{
                    if let Some(simple) = &self.#field_name {
                      attr_list.push((
                        #s_field_name.to_string(),
                        crate::ast::AttriValue::Simple(
                          crate::ast::SimpleAttri::to_wrapper(simple),
                        ),
                      ));  
                    }
                  };
                  parser_arm = quote!{
                    let simple_res: _;
                    (input,simple_res) = <_ as crate::ast::SimpleAttri>::nom_parse(input,line_num)?;
                    match simple_res {
                      Ok(simple) => {
                        res.#field_name=Some(simple);
                      },
                      Err((e,attri)) => {
                        println!("Line={}; Key={}; Value={:?}; Err={}",line_num,key,attri,e);
                        res.add_undefine_attri(key,attri);
                      },
                    }
                  };
                },
                None => {
                  // for xxx
                  to_wrapper = quote!{
                    attr_list.push((
                      #s_field_name.to_string(),
                      crate::ast::AttriValue::Simple(
                        crate::ast::SimpleAttri::to_wrapper(&self.#field_name),
                      ),
                    ));
                  };
                  parser_arm = quote!{
                    let simple_res: _;
                    (input,simple_res) = <_ as crate::ast::SimpleAttri>::nom_parse(input,line_num)?;
                    match simple_res {
                      Ok(simple) => {
                        res.#field_name=simple;
                      },
                      Err((e,attri)) => {
                        println!("Line={}; Key={}; Value={:?}; Err={}",line_num,key,attri,e);
                        res.add_undefine_attri(key,attri);
                      },
                    }
                  };
                },
              }
            },
            // AttriType::SimpleOption => {
              
            // },
            AttriType::Complex => {
              to_wrapper = quote!{
                if let Some(wrapper) = crate::ast::ComplexAttri::to_wrapper(&self.#field_name){
                  attr_list.push((
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Complex(wrapper),
                  ));
                }
                // if !crate::ast::ComplexAttri::is_empty(&self.#field_name) {
                //   attr_list.push((
                //     #s_field_name.to_string(),
                //     crate::ast::AttriValue::Complex(
                //       crate::ast::ComplexAttri::to_wrapper(&self.#field_name),
                //     ),
                //   ));  
                // }
              };
              parser_arm = quote!{
                let complex_res: _;
                (input,complex_res) = <_ as crate::ast::ComplexAttri>::nom_parse(input,line_num)?;
                match complex_res {
                  Ok(complex) => res.#field_name=complex,
                  Err((e,attri)) => {
                    println!("Line={}; Key={}; Value={:?}; Err={}",line_num,key,attri,e);
                    res.add_undefine_attri(key,attri);
                  },
                }
              };
            },
            AttriType::Group => {
              to_wrapper = quote!{
                attr_list.extend(self.#field_name.iter().map(
                  |group|(
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Group(
                      crate::ast::GroupAttri::to_wrapper(group),
                    )
                  )
                ).collect::<Vec<(String, crate::ast::AttriValue)>>());
              };
              parser_arm = quote!{
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
            },
            AttriType::GroupHashed => {
              to_wrapper = quote!{
                attr_list.extend(self.#field_name.iter().map(
                  |(idx,group)|(
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Group(
                      crate::ast::GroupAttri::to_wrapper(group)
                    )
                  )
                ).collect::<Vec<(String, crate::ast::AttriValue)>>());
              };
              parser_arm = quote!{
                let group_res: _;
                (input,group_res) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
                match group_res{
                  Ok(group) => {
                    if let Some(old) = res.#field_name.insert(
                      <_ as crate::ast::HashedGroup>::idx_clone(&group),
                      group,
                    ){
                    let e = crate::ast::IdxError::RepeatIdx; 
                    println!("Line={}, error={}",line_num,e);
                    }
                  },
                  Err(e) => {
                    println!("Line={}, error={}",line_num,e);
                    ();
                  },
                }
                let n: usize;
                (input,n) = crate::ast::parser::comment_space_newline(input)?;
                *line_num+=n;
              };
            },
          };
          parser_arms = quote!{
            #parser_arms
            #s_field_name => {
              #parser_arm
            },
          };
          to_wrappers = quote!{
            #to_wrappers
            #to_wrapper
          }
        }
      }
    }
  }
  let impl_set = if hashed {
    quote!{type Set=std::collections::HashMap<<Self as crate::ast::HashedGroup>::Idx,Self>;}
  }else{
    quote!{type Set=Vec<Self>;}
  };
  let change_idx_return = if hashed {
    quote!{
      match crate::ast::HashedGroup::gen_idx(&res,title){
        Ok(idx) => {
          res._idx = Box::new(idx);
          return Ok((input,Ok(res)));
        },
        Err(e) => {
          return Ok((input,Err(e)));
        },
      }
    }
  }else{
    quote!{return Ok((input, Ok(res)));}
  };
  let to_wrapper_title = if hashed {
    quote!{ crate::ast::HashedGroup::title(self) }
  }else{
    quote!{ vec![] }
  };
  let impl_group = quote!{
    impl crate::ast::GroupAttri for #name {
      #impl_set
      #[inline]
      fn add_undefine_attri(&mut self, key: &str, attri: crate::ast::AttriValue) {
        self._undefined.push((key.to_owned(),attri))
      }

      fn to_wrapper(&self) -> crate::ast::GroupWrapper {
        let mut attr_list: Vec<(String, crate::ast::AttriValue)> = self._undefined.clone();
        #to_wrappers
        crate::ast::GroupWrapper{ 
          title: #to_wrapper_title ,
          attr_list,
        }
      }
      fn nom_parse<'a>(
        i: &'a str, line_num: &mut usize
      ) -> nom::IResult<&'a str, Result<Self,crate::ast::IdxError>, nom::error::Error<&'a str>> {
        let (mut input,title) = crate::ast::parser::title(i,line_num)?;
        let mut res = Self::default();
        loop {
          match crate::ast::parser::key(input){
            Err(nom::Err::Error(_)) => {
              (input,_) = crate::ast::parser::end_group(input)?;
              #change_idx_return
            },
            Err(e) => return Err(e),
            Ok((_input,key)) => {
              input = _input;
              match key {
                #parser_arms
                _ => {
                  let undefine: crate::ast::AttriValue;
                  (input,undefine) = crate::ast::parser::undefine(input,line_num)?;
                  res.add_undefine_attri(key, undefine);
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
  Ok(impl_group)
}
  
#[derive(Debug)]
enum AttriType {
  Simple,
  // SimpleOption,
  Complex,
  Group,
  GroupHashed,
  Unkown(String),
}
  
fn parse_field_attrs(field_attrs: &Vec<Attribute>) -> Option<AttriType>{
  for attri in field_attrs.into_iter(){
    if let Some(seg_title) = attri.path.segments.first(){
      if "arrti_type"== &seg_title.ident.to_string(){
        if let Ok(NestedMeta::Meta(meta))=attri.parse_args::<NestedMeta>(){
          if let Some(seg_type) = meta.path().segments.first(){
            let type_str = seg_type.ident.to_string();
            match type_str.as_str(){
              "simple" => return Some(AttriType::Simple),
              // "simple" => return Some(AttriType::SimpleOption),
              "complex" => return Some(AttriType::Complex),
              "group" => return Some(AttriType::Group),
              "group_hashed" => return Some(AttriType::GroupHashed),
              _ => return Some(AttriType::Unkown(type_str)),
            }
          }
        }
      }
    }
  }
  return None;
}

fn extract_type_from_option(ty: &syn::Type) -> Option<&syn::Type> {
  use syn::{GenericArgument, Path, PathArguments, PathSegment};

  fn extract_type_path(ty: &syn::Type) -> Option<&Path> {
      match *ty {
          syn::Type::Path(ref typepath) if typepath.qself.is_none() => Some(&typepath.path),
          _ => None,
      }
  }

  // TODO store (with lazy static) the vec of string
  // TODO maybe optimization, reverse the order of segments
  fn extract_option_segment(path: &Path) -> Option<&PathSegment> {
      let idents_of_path = path
          .segments
          .iter()
          .into_iter()
          .fold(String::new(), |mut acc, v| {
              acc.push_str(&v.ident.to_string());
              acc.push('|');
              acc
          });
      vec!["Option|", "std|option|Option|", "core|option|Option|"]
          .into_iter()
          .find(|s| &idents_of_path == *s)
          .and_then(|_| path.segments.last())
  }

  extract_type_path(ty)
      .and_then(|path| extract_option_segment(path))
      .and_then(|path_seg| {
          let type_params = &path_seg.arguments;
          // It should have only on angle-bracketed param ("<String>"):
          match *type_params {
              PathArguments::AngleBracketed(ref params) => params.args.first(),
              _ => None,
          }
      })
      .and_then(|generic_arg| match *generic_arg {
          GenericArgument::Type(ref ty) => Some(ty),
          _ => None,
      })
}
  