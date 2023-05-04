use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Attribute, NestedMeta};
use syn::{Data, Fields};

#[proc_macro_derive(NameIdx)]
pub fn macro_name_idx(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let name = &ast.ident;
  let idx_name = format_ident!("{}Idx", name);
  let toks = quote! {
    /// Identitied by its Name.
    #[derive(Debug,Default,Hash,Eq,PartialEq)]
    pub struct #idx_name {
      // Name.
      pub name: String,
    }
    impl crate::ast::GroupIdx for #idx_name {
      #[inline]
      fn title(&self) -> Vec<String> {
        vec![self.name.clone()]
      }
    }
    impl crate::ast::HashedGroup for #name {
      type Idx=#idx_name;
      #[inline]
      fn idx<'a>(&self, mut title: Vec<&'a str>)->Result<Self::Idx,crate::ast::IdxError<'a>>{
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
pub fn macro_group(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group_inner(&ast, false).unwrap_or_else(|err| err.to_compile_error().into());
  toks.into()
}

#[proc_macro_derive(GroupHashed, attributes(arrti_type))]
pub fn macro_group_hashed(input: TokenStream) -> TokenStream {
  let ast = parse_macro_input!(input as DeriveInput);
  let toks = group_inner(&ast, true).unwrap_or_else(|err| err.to_compile_error().into());
  toks.into()
}

use proc_macro2::Span;
fn group_inner(ast: &DeriveInput, hashed: bool) -> syn::Result<TokenStream>{
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
              format!("Unspported type: {s}, spport: [simple,simple_multi,complex,group,group_hashed]")
            )),
            AttriType::Simple => {
              to_wrapper = quote!{
                if let Some(simple) = self.#field_name {
                  attr_list.push((
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Simple(
                      crate::ast::SimpleAttri::to_wrapper(&simple),
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
            AttriType::SimpleMulti => {
              to_wrapper = quote!{
                if let Some(simple) = self.#field_name {
                  attr_list.push((
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Simple(
                      crate::ast::SimpleAttri::to_wrapper(&simple),
                    ),
                  ));  
                }
              };
              parser_arm = quote!{
              };
            },
            AttriType::Complex => {
              to_wrapper = quote!{
                if !crate::ast::ComplexAttri::is_empty(&self.#field_name) {
                  attr_list.push((
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Complex(
                      crate::ast::ComplexAttri::to_wrapper(&self.#field_name),
                    ),
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
              to_wrapper = quote!{
                attr_list.extend(self.#field_name.iter().map(
                  |group|(
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Group(
                      crate::ast::GroupAttri::to_wrapper(
                        group,
                        vec![],
                      ),
                    )
                  )
                ).collect::<Vec<(String, crate::ast::AttriValue)>>());
              };
              parser_arm = quote!{
                let group: _;
                (input,(_,group)) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
                res.#field_name.push(group);
                let n: usize;
                (input,n) = crate::ast::parser::space_newline(input)?;
                *line_num+=n;
              };
            },
            AttriType::GroupHashed => {
              to_wrapper = quote!{
                attr_list.extend(self.#field_name.iter().map(
                  |(idx,group)|(
                    #s_field_name.to_string(),
                    crate::ast::AttriValue::Group(
                      crate::ast::GroupAttri::to_wrapper(
                        group,
                        crate::ast::GroupIdx::title(idx),
                      ),
                    )
                  )
                ).collect::<Vec<(String, crate::ast::AttriValue)>>());
              };
              parser_arm = quote!{
                let group: _;
                let title: _;
                (input,(title,group)) = <_ as crate::ast::GroupAttri>::nom_parse(input, line_num)?;
                match crate::ast::HashedGroup::idx(&group,title){
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
  let impl_group = quote!{
    impl crate::ast::GroupAttri for #name {
      #impl_set
      #[inline]
      fn add_undefine_attri(&mut self, key: &str, attri: crate::ast::AttriValue) {
        self._undefined.push((key.to_owned(),attri))
      }

      fn to_wrapper(&self, title: Vec<String>) -> crate::ast::GroupWrapper {
        let mut attr_list: Vec<(String, crate::ast::AttriValue)> = self._undefined.clone();
        #to_wrappers
        crate::ast::GroupWrapper{ 
          title,
          attr_list,
        }
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
                #parser_arms
                _ => {
                  let undefine: crate::ast::AttriValue;
                  (input,undefine) = crate::ast::parser::undefine(input,line_num)?;
                  res.add_undefine_attri(key, undefine);
                  let n: usize;
                  (input,n) = crate::ast::parser::space_newline(input)?;
                  *line_num+=n;
                },
              }
            }
          }
        }
      }
    }
  };
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
        if let Ok(NestedMeta::Meta(meta))=attri.parse_args::<NestedMeta>(){
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
  return None;
}
