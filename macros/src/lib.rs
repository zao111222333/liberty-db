use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Attribute, NestedMeta};
use syn::{Data, Fields};

#[proc_macro_derive(NameIdx)]
pub fn name_idx_macro(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let name = &ast.ident;
  let idx_name = format_ident!("{}Idx", name);
  let toks = quote! {
    /// Identitied by its Name.
    #[derive(Debug,Default,Hash,Eq, PartialEq)]
    pub struct #idx_name {
      // Name.
      pub name: String,
    }
    impl crate::ast::HashedGroup for #name {
      type Idx=#idx_name;
      fn idx<'a>(s: &Self, mut title: Vec<&'a str>)->Result<Self::Idx,crate::ast::IdxError<'a>> {
        let l=title.len();
        if l!=1{
            return Err(crate::ast::IdxError::TitleLenMismatch(1,l,title));
        }
        Ok(Self::Idx { name: title.pop().unwrap().to_string() })
      }
    }
  };
  toks.into()
}

#[proc_macro_derive(Group, attributes(arrti_type))]
pub fn group_macro(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks =from_string_inner(&ast, false).unwrap_or_else(|err| err.to_compile_error().into());
  println!("{}", toks);
  toks.into()
}

#[proc_macro_derive(GroupHashed, attributes(arrti_type))]
pub fn group_macro1(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks =from_string_inner(&ast, true).unwrap_or_else(|err| err.to_compile_error().into());
  println!("{}", toks);
  toks.into()
}

use proc_macro2::Span;
fn from_string_inner(ast: &DeriveInput,hashed: bool) -> syn::Result<TokenStream>{
  let name = &ast.ident;
  let st = match &ast.data {
    Data::Struct(s) => s,
    _ => return Err(syn::Error::new(Span::call_site(), "This macro only supports struct.")),
  };
  let mut arms = quote!{};
  if let Fields::Named(named) =  &st.fields{
    let fields = &named.named;
    for field in fields.into_iter(){
      if let (Some(field_name),field_attrs) = (&field.ident,&field.attrs){
        if let Some(arrti_type) = parse_field_attrs(field_attrs){
          let arm: _;
          match arrti_type{
              AttriType::Simple => arm = 
              quote!{
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
              },
              AttriType::SimpleMulti => arm = 
              quote!{
                todo()!
              },
              AttriType::Complex => arm = 
              quote!{
                let complex_res: _;
                (input,complex_res) = <_ as crate::ast::ComplexAttri>::nom_parse(input,line_num)?;
                match complex_res {
                  Ok(complex) => res.#field_name=complex,
                  Err((e,attri)) => {
                    println!("Line={}; Key={}; Value={:?}; Err={}",line_num,key,attri,e);
                    res.add_undefine_attri(key,attri);
                  },
                }
              },
              AttriType::Group => arm = 
              quote!{
                let group: _;
                (input,(_,group)) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
                res.#field_name.push(group);
                let n: usize;
                (input,n) = crate::ast::parser::space_newline(input)?;
                *line_num+=n;
              },
              AttriType::GroupHashed => arm = quote!{
                let group: _;
                let title: _;
                (input,(title,group)) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
                match <_ as crate::ast::HashedGroup>::idx(&group, title){
                  Ok(idx) => {
                    if let Some(old) = res.#field_name.insert(idx, group) {
                      let e = crate::ast::IdxError::RepeatIdx; 
                      println!("Line={}, error={}",line_num,e);
                    }
                  },
                  Err(e) => println!("Line={}, error={}",line_num,e),
                }
                let n: usize;
                (input,n) = crate::ast::parser::space_newline(input)?;
                *line_num+=n;
              },
              AttriType::Unkown(s) => return Err(syn::Error::new(Span::call_site(), 
                                        format!("Unspported type: {s}, spport: [simple,simple_multi,complex,group,group_hashed]")
                                        )),
          }
          let s_field_name = field_name.to_string();
          arms = quote!{
            #arms
            #s_field_name => {
              #arm
            },
          };
        }
      }
    }
  }
  let impl_set = if hashed {
    quote!{type Set=std::collections::HashMap<<Self as crate::ast::HashedGroup>::Idx,Self>;}
  }else{
    quote!{type Set=Vec<Self>;}
  };
  let impl_group = quote!{
    impl crate::ast::GroupAttri for #name {
      #impl_set

      fn add_undefine_attri(&mut self, key: &str, attri: crate::ast::AttriValue) {
        self._undefined.push((key.to_owned(),attri))
      }
      fn nom_parse<'a>(
        i: &'a str, line_num: &mut usize
      ) -> nom::IResult<&'a str, (Vec<&'a str>,Self), nom::error::Error<&'a str>> {
        let (mut input,title) = crate::ast::parser::title(i,line_num)?;
        let mut res = Self::default();
        loop {
          match crate::ast::parser::key(input){
            Err(nom::Err::Error(_)) => {
              (input,_) = crate::ast::parser::end_group(input)?;
              return Ok((input, (title,res)))
            },
            Err(e) => return Err(e),
            Ok((_input,key)) => {
              input = _input;
              match key {
                #arms
                _ => {
                  let undefine: crate::ast::AttriValue;
                  (input,undefine) = crate::ast::parser::undefine(input)?;
                  println!("Line={}; Undefinde Error; Key={};",line_num,key);
                  res.add_undefine_attri(key, undefine)
                },
              }
            }
          }
        }
      }
    }
  };
  // impl_group = quote!{
  //   #impl_group
  //   struct xxxx{}
  // };
  // todo!()
  // Err(syn::Error::new(Span::call_site(), "This macro only supports struct."))
  Ok(quote!{
      #impl_group
  }.into())
}

#[derive(Debug)]
enum AttriType {
  Simple,
  SimpleMulti,
  Complex,
  Group,
  GroupHashed,
  Unkown(String),
}

fn parse_field_attrs(field_attrs: &Vec<Attribute>) -> Option<AttriType>{
  for attri in field_attrs.into_iter(){
    if let Some(seg_title) = attri.path.segments.first(){
      if "arrti_type"== &seg_title.ident.to_string(){
        if let Ok(nested)=attri.parse_args::<NestedMeta>(){
          if let NestedMeta::Meta(meta) = nested{
            if let Some(seg_type) = meta.path().segments.first(){
              let type_str = seg_type.ident.to_string();
              match type_str.as_str(){
                "simple" => return Some(AttriType::Simple),
                "simple_multi" => return Some(AttriType::SimpleMulti),
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
  }
  return None;
}