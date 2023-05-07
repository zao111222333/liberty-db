use proc_macro2::Span;
use quote::quote;
use syn::{DeriveInput, Attribute, NestedMeta};
use syn::{Data, Fields};

pub(crate) fn inner(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream>{
  let name = &ast.ident;
  let st = match &ast.data {
    Data::Struct(s) => s,
    _ => return Err(syn::Error::new(Span::call_site(), "This macro only supports struct.")),
  };
  let mut parser_arms = quote!{};
  let mut to_wrappers = quote!{};
  if let Fields::Named(named) =  &st.fields{
    let fields = &named.named;
    let mut hashed = false;
    for field in fields.into_iter(){
      if let Some(id) = &field.ident{
        if "_idx" == &id.to_string(){
          hashed = true;
          break;
        }
      }
    }
    for field in fields.into_iter(){
      if let (Some(field_name),field_attrs) = (&field.ident,&field.attrs){
        if let Some(arrti_type) = parse_field_attrs(field_attrs){
          let s_field_name = field_name.to_string();
          let to_wrapper: _;
          let parser_arm: _;
          match arrti_type{
            AttriType::Unkown(s) => return Err(syn::Error::new(Span::call_site(), 
              format!("Unsupported type: {s}, support: [simple,complex,group]")
            )),
            AttriType::Simple => {
              match extract_type(&field.ty){
                Some((Some(FieldType::Option),_)) => {
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
                _ => {
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
            AttriType::Complex => {
              to_wrapper = quote!{
                if let Some(wrapper) = crate::ast::ComplexAttri::to_wrapper(&self.#field_name){
                  attr_list.push((
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Complex(wrapper),
                  ));
                }
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
              match extract_type(&field.ty){
                Some((Some(FieldType::Vector),_)) => {
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
                Some((Some(FieldType::HashMap),_)) => {
                  to_wrapper = quote!{
                    attr_list.extend(self.#field_name.iter().map(
                      |(_,group)|(
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
                      },
                    }
                    let n: usize;
                    (input,n) = crate::ast::parser::comment_space_newline(input)?;
                    *line_num+=n;
                  };
                },
                Some((Some(FieldType::Option),_)) => {
                  to_wrapper = quote!{
                    if let Some(group) = &self.#field_name {
                      attr_list.push((
                        #s_field_name.to_string(),
                        crate::ast::AttriValue::Group(
                          crate::ast::GroupAttri::to_wrapper(group),
                        ),
                      ));  
                    }
                  };
                  parser_arm = quote!{
                    let group_res: _;
                    (input,group_res) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
                    match group_res{
                      Ok(group) => {
                        if let Some(old) = res.#field_name{
                          let e = crate::ast::IdxError::RepeatIdx; 
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
                },
                Some((None,ty)) => {return Err(syn::Error::new(Span::call_site(), 
                  format!("Unsupported field={}, type={}",s_field_name,ty)
                ))},
                None => {return Err(syn::Error::new(Span::call_site(), 
                  format!("Unsupported field={}, type={:?}",s_field_name,&field.ty)
                ))},
              }
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
  }else{
    Err(syn::Error::new(Span::call_site(), 
                  format!("Can not find NamedField")
                ))
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
  
fn parse_field_attrs(field_attrs: &Vec<Attribute>) -> Option<AttriType>{
  for attri in field_attrs.into_iter(){
    if let Some(seg_title) = attri.path.segments.first(){
      if "arrti_type"== &seg_title.ident.to_string(){
        if let Ok(NestedMeta::Meta(meta))=attri.parse_args::<NestedMeta>(){
          if let Some(seg_type) = meta.path().segments.first(){
            let type_str = seg_type.ident.to_string();
            match type_str.as_str(){
              "simple" => return Some(AttriType::Simple),
              "complex" => return Some(AttriType::Complex),
              "group" => return Some(AttriType::Group),
              // "group_hashed" => return Some(AttriType::GroupHashed),
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
  HashMap,
  Vector,
  Option,
}

/// https://stackoverflow.com/questions/55271857/how-can-i-get-the-t-from-an-optiont-when-using-syn
fn extract_type(ty: &syn::Type) -> Option<(Option<FieldType>,String)> {
  match *ty {
    syn::Type::Path(ref typepath) if typepath.qself.is_none() => {
      let idents_of_path = typepath.path
      .segments
      .iter()
      .into_iter()
      .fold(String::new(), |mut acc, v| {
          acc.push_str(&v.ident.to_string());
          acc.push('|');
          acc
      });
      if let Some(_)=vec!["Option|", "std|option|Option|", "core|option|Option|","option|Option|"]
        .into_iter()
        .find(|s| &idents_of_path == s){
          return Some((Some(FieldType::Option),idents_of_path));
        }
      if let Some(_)=vec!["HashMap|","std|collections|HashMap|"]
        .into_iter()
        .find(|s| &idents_of_path == s){
          return Some((Some(FieldType::HashMap),idents_of_path));
        }
        
      if let Some(_)=vec!["Vec|", "alloc|vec|Vec|"]
        .into_iter()
        .find(|s| &idents_of_path == s){
          return Some((Some(FieldType::Vector),idents_of_path));
        }
      return Some((None,idents_of_path));
    },
      _ => None,
  }
}

