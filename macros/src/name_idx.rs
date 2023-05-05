use proc_macro2::Span;
use proc_macro::TokenStream;
use quote::{quote, format_ident};
use syn::{parse_macro_input, DeriveInput, Attribute, NestedMeta};
use syn::{Data, Fields};


pub(crate) fn inner(ast: &DeriveInput) -> syn::Result<proc_macro2::TokenStream>{
    let st = match &ast.data {
      Data::Struct(s) => s,
      _ => return Err(syn::Error::new(Span::call_site(), "This macro only supports struct.")),
    };
    let idx_len = match find_idx_len(st){
      Some(idx_len) => idx_len,
      None => return Err(syn::Error::new(Span::call_site(), "Can not find `_idx: Box<<Self as HashedGroup>::Idx>`")),
    };
    let name = &ast.ident;
    let idx_name = format_ident!("{}Idx", name);
    let fn_idx = quote!{
      #[inline]
      fn idx(&self) -> &Self::Idx {
        self._idx.as_ref()
      }
      #[inline]
      fn idx_clone(&self) -> Self::Idx{
        (*self._idx).clone()
      }
    };
    let toks = match idx_len{
      IdxLen::Num(0) => return Err(syn::Error::new(Span::call_site(), "`idx_len` should larger than `1`")),
      IdxLen::Num(1) => quote! {
        /// Identitied by its Name.
        #[derive(Debug,Default,Clone,Hash,Eq,PartialEq)]
        pub struct #idx_name {
          // Name.
          pub name: String,
        }
        impl crate::ast::HashedGroup for #name {
          type Idx=#idx_name;
          #[inline]
          fn title(&self) -> Vec<String>{
            vec![self._idx.name.clone()]
          }
          #fn_idx
          #[inline]
          fn gen_idx(&self, mut title: Vec<String>) -> Result<Self::Idx,crate::ast::IdxError>{
            let l=title.len();
            if l!=1{
                return Err(crate::ast::IdxError::TitleLenMismatch(1,l,title));
            }
            if let Some(name) = title.pop(){
              Ok(Self::Idx { name })
            }else{
              return Err(crate::ast::IdxError::Other("Unkown pop error".into()));
            }
          }
        }
      },
      IdxLen::Num(c) => quote! {
        /// Identitied by its Name.
        #[derive(Debug,Default,Clone,Hash,Eq,PartialEq)]
        pub struct #idx_name {
          // Name.
          pub name: [String;#c],
        }
        impl crate::ast::HashedGroup for #name {
          type Idx=#idx_name;
          #[inline]
          fn title(&self) -> Vec<String>{
            self._idx.name.clone().to_vec()
          }
          #fn_idx
          #[inline]
          fn gen_idx(&self, title: Vec<String>) -> Result<Self::Idx,crate::ast::IdxError>{
            let l=title.len();
            if l!=#c{
                return Err(crate::ast::IdxError::TitleLenMismatch(#c,l,title));
            }
            match TryInto::<[String; #c ]>::try_into(title){
              Ok(name) => Ok(Self::Idx { name }),
              Err(e) => Err(crate::ast::IdxError::Other(format!("try_into error: {:?}",e))),
            }
          }
        }
      },
      IdxLen::Unkown => quote! {
        /// Identitied by its Name.
        #[derive(Debug,Default,Clone,Hash,Eq,PartialEq)]
        pub struct #idx_name {
          // Name.
          pub name: Vec<String>,
        }
        impl crate::ast::HashedGroup for #name {
          type Idx=#idx_name;
          #[inline]
          fn title(&self) -> Vec<String>{
            self._idx.name.clone()
          }
          #fn_idx
          #[inline]
          fn gen_idx(&self, title: Vec<String>) -> Result<Self::Idx,crate::ast::IdxError>{
            Ok(Self::Idx { name: title })
          }
        }
      },
    };
    Ok(toks)
  }
  
  #[derive(Debug)]
  enum IdxLen {
    Num(usize),
    Unkown,
  }
  
  fn find_idx_len(st: &syn::DataStruct) -> Option<IdxLen>{
    if let Fields::Named(named) =  &st.fields{
      let fields = &named.named;
      for field in fields.into_iter(){
        if let Some(id) = &field.ident{
          if "_idx" == &id.to_string(){
            return Some(parse_idx_len(&field.attrs));
          }
        }
      }
    }
    return None;
  }
  
  fn parse_idx_len(field_attrs: &Vec<Attribute>) -> IdxLen{
    for attri in field_attrs.into_iter(){
      if let Some(seg_title) = attri.path.segments.first(){
        if "idx_len"== &seg_title.ident.to_string(){
          if let Ok(NestedMeta::Lit(syn::Lit::Int(n)))=attri.parse_args::<NestedMeta>(){
            match n.base10_parse::<usize>(){
              Ok(n) => return IdxLen::Num(n),
              Err(_) => return IdxLen::Unkown,
            }
          }
        }
      }
    }
    return IdxLen::Unkown;
  }
  